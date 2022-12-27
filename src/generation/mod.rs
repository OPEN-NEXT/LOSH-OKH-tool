// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod language;

use chrono::Datelike;
use projvar::environment::Environment;
use projvar::var::Key;

use chrono::{DateTime, Utc};
use regex::Regex;
use relative_path::{RelativePath, RelativePathBuf};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use url::Url;

use crate::formats::v2;
use crate::formats::v2::SubMosh;
use crate::formats::SerError;
use crate::macros::rgx;
use crate::{dir, file_types, license};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to find a value for projvar key '{0:?}'.")]
    PVKeyNotFound(Key),

    #[error("Failed to read or write, probably from/to the file-system.")]
    Io(#[from] std::io::Error),

    #[error("Misc error.")]
    General(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Failed to serialize to TOML/YAML/similar.")]
    Serialization(#[from] SerError),

    #[error("Failed to initialize a git repo.")]
    Git2RepoInit(#[from] git2::Error),

    #[error("We require a project to be a git repo to be able to generate a manifest file.")]
    NotAGitRepo(PathBuf),

    #[error("Failed to convert string to UTF-8.")]
    NonUtf8(#[from] std::string::FromUtf8Error),

    #[error("Failed to cast a path to a reltive one.")]
    PathNotRelative(#[from] relative_path::FromPathError),

    #[error("Failed to cast a string to a URL.")]
    NotaUrl(#[from] url::ParseError),
}

type Res<O> = Result<O, Error>;
type ORelPath = Option<RelativePathBuf>;

#[inline]
fn pv(environment: &projvar::environment::Environment, key: Key) -> Res<String> {
    Ok(environment
        .output
        .get(key)
        .ok_or(Error::PVKeyNotFound(key))?
        .1
        .clone())
}

fn is_release_version(version: &str) -> bool {
    semver::Version::parse(version)
        .map(|vers| vers.pre.is_empty() && vers.build.is_empty())
        .unwrap_or(false)
}

pub fn run_projvar(proj_root: &Path) -> Res<projvar::environment::Environment> {
    let fail_on_missing = false;
    let settings = projvar::settings::Settings {
        repo_path: Some(proj_root.to_path_buf()),
        required_keys: projvar::var::default_keys().clone(),
        date_format: projvar::tools::git::DATE_FORMAT.to_owned(),
        overwrite: projvar::settings::Overwrite::All,
        fail_on: projvar::settings::FailOn::from(fail_on_missing),
        show_retrieved: projvar::settings::ShowRetrieved::No,
        hosting_type: projvar::tools::git_hosting_provs::HostingType::Unknown,
        only_required: true,
        key_prefix: None,
        verbosity: (
            projvar::settings::Verbosity::None,
            projvar::settings::Verbosity::None,
        ),
    };
    log::trace!("Created Settings.");
    let mut environment = projvar::environment::Environment::new(settings);

    // environment
    //     .vars
    //     .insert("LICENSE".to_owned(), "AGPL-2.0-or-later".to_owned());

    let sources = projvar::sources::default_list(proj_root);
    let sinks: Vec<Box<dyn projvar::sinks::VarSink>> = vec![];

    projvar::process::run(&mut environment, sources, sinks)?;

    Ok(environment)
}

pub fn find_root_files(proj_root: &Path) -> (ORelPath, ORelPath, ORelPath) {
    let root_file_filters = &[
        rgx!(r#"README.*(\.(md|markdown))?"#),
        rgx!(r#"[Bb](ill)?[-_]?[Oo](f)?[-_]?[Mm](aterials)?"#),
        rgx!(r#"CONTRIBUTI(NG|ON)?(\.(md|markdown))?"#),
    ];
    let found_files = dir::scan(proj_root, false, root_file_filters, Path::file_name);
    let single_found_files = found_files.iter().map(|fnds| {
        let mut sorted = fnds.clone();
        sorted.sort_by_key(|pth| pth.as_os_str().len());
        sorted.get(0).map(|p| p.display().to_string())
    });
    let mut rel_paths = single_found_files
        .map(Option::unwrap)
        .map(RelativePathBuf::from);

    (rel_paths.next(), rel_paths.next(), rel_paths.next())
}

pub fn find_rec_files(proj_root: &Path) -> Vec<Vec<RelativePathBuf>> {
    let file_ext_filters = &[
        rgx!(r#"^(png|jpg|jpeg|gif|svg)$"#),
        rgx!(r#"^toml$"#),
        &Regex::new(file_types::RS_CAD).unwrap(),
        &Regex::new(file_types::RS_PCB).unwrap(),
    ]; // TODO Write the second filter and use it
    let found_rec_files = dir::scan(proj_root, true, file_ext_filters, Path::extension);
    let found_rec_files = found_rec_files.iter().map(|fnds| {
        let mut sorted = fnds.clone();
        sorted.sort_by_key(|pth| pth.as_os_str().len());
        sorted
    });
    let rec_files_groups: Result<_, _> = found_rec_files
        .into_iter()
        .map(|rec_files_group| {
            rec_files_group
                .iter()
                .map(RelativePathBuf::from_path)
                .collect()
        })
        .collect();
    rec_files_groups.unwrap()
}

fn first_commit(repo: &git2::Repository) -> Res<git2::Commit<'_>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;
    Ok(repo.find_commit(revwalk.next().unwrap()?)?)
}

fn find_parts(
    rec_files_groups: &[Vec<RelativePathBuf>],
    module_dir: &Path,
    environment: &Environment,
    overwrite: bool,
) -> Res<Vec<SubMosh>> {
    // will map a dir (making up a part/sub-module)
    // to the relevant file-names within:
    // okh.toml, CAD and PCB files
    // let part_dirs: HashMap<PathBuf, HashSet<OsString>> = HashMap::new();
    let mut sub_part_dirs: HashMap<RelativePathBuf, HashSet<RelativePathBuf>> = HashMap::new();

    // for file_type in file_types::CAD {
    //     println!("XXX ext: {}", file_type.extension);
    // }
    let okh_toml_name = "okh.toml";
    let okh_toml_name_rel_path = RelativePathBuf::from("okh.toml");
    let okh_toml_name_os = OsStr::new(okh_toml_name);
    let cwd = RelativePathBuf::new();
    // let some_okh_toml_name = Some(okh_toml_name);
    for toml_path in &rec_files_groups[1] {
        // let toml_path = Path::new(toml_file);
        // Its an OKH TOML and not the root one.
        if toml_path
            .file_name()
            .filter(|f_name| *f_name == okh_toml_name_os)
            .is_some()
        {
            let sub_part_dir = toml_path.parent();
            if let Some(sub_part_dir) = sub_part_dir {
                if sub_part_dir != cwd {
                    println!("XXX part_dir: {sub_part_dir}");
                    sub_part_dirs
                        .entry(sub_part_dir.to_relative_path_buf())
                        .or_insert_with(HashSet::new)
                        .insert(okh_toml_name_rel_path.clone());
                }
            }
        }
    }
    for design_files_group in [&rec_files_groups[2], &rec_files_groups[3]] {
        for design_path in design_files_group {
            // let design_path = Path::new(design_file);
            // println!("XXX ext: {}", file_type.extension);
            if let Some(sub_part_dir) = design_path.parent() {
                sub_part_dirs
                    .entry(sub_part_dir.to_relative_path_buf())
                    .or_insert_with(HashSet::new)
                    // .insert(design_path.file_name().unwrap());
                    .insert(RelativePathBuf::from_path(PathBuf::from(
                        design_path.file_name().unwrap(),
                    ))?);
            }
        }
    }

    let mut part = vec![];
    for (sub_part_dir, files) in sub_part_dirs {
        let manifest_file = sub_part_dir.join(okh_toml_name);
        let sub_part_path = sub_part_dir.to_path(module_dir);
        if !files.contains(&okh_toml_name_rel_path) {
            // generate child okh.toml file
            if is_git_submodule(&sub_part_path) {
                okh_losh_toml_part(&sub_part_path, &cwd, None, overwrite)?; // TODO is overwrite inheritance ok/save? :/
            } else {
                okh_losh_toml_part(module_dir, &sub_part_dir, Some(environment), overwrite)?;
                // TODO is overwrite inheritance ok/save? :/
            }
        }
        let name = sub_part_dir.file_name().map(str::to_owned);
        // println!("XXX ext: {}", file_type.extension);
        // part.push(SubMosh {
        //     name,
        //     manifest_file: Some(manifest_file),
        //     repo: None, // TODO Use this repos Url, or the submodules one, if this module is a git repo by itsself
        // });
        let image = vec![];
        let tsdc = None;
        let source = vec![];
        let export = vec![];
        let auxiliary = vec![];
        let inner_part = vec![];
        part.push(v2::SubMosh {
            name,
            // manifest_file,
            // repo,
            image,
            tsdc,
            source,
            export,
            auxiliary,
            part: inner_part,
        });
    }

    Ok(part)
}

fn generate_data(module_dir: &Path, environment: &Environment, overwrite: bool) -> Res<v2::Okh> {
    let license = license::ensure_spdx_license_id(&pv(environment, Key::License)?);

    let documentation_language = language::identify(&fs::read_to_string("README.md")?); // TODO HACK look for README* instead!

    let version = pv(environment, Key::Version)?;
    // projvar::validator::get(Key::Version)(release);
    if !is_release_version(&version) {
        log::warn!("You are not on a release version: {}", version);
    }

    let now: DateTime<Utc> = Utc::now();
    // println!("UTC now is: {}", now);
    // println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    // println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    // println!(
    //     "UTC now in a custom format is: {}",
    //     now.format("%a %b %e %T %Y")
    // );
    let timestamp = Some(now.to_rfc2822());

    // let licensor = "ANONYMOUS".to_owned();
    // NOTE FIXME - As licensor, we use the git user of the first commit; but what about the case of a fork?
    // let repo = git2::Repository::open(proj_root)?;
    let repo = environment
        .repo()
        .ok_or_else(|| Error::NotAGitRepo(module_dir.to_path_buf()))?
        .inner();
    // let repo = repo.inner();
    let first_commit = first_commit(repo)?;
    // println!("first commit msg: {}", first_commit.message().unwrap());
    let git_author = first_commit.author().to_string();
    let licensor = git_author;

    let (readme, bom, contribution_guide) = find_root_files(module_dir); // TODO Use git list if git repo, otherwise filesystem list - does projvar already have this, or only osh-tool (Nim :/) ?

    let rec_files_groups = find_rec_files(module_dir); // TODO Use git list if git repo, otherwise filesystem list - does projvar already have this, or only osh-tool (Nim :/) ?
    let image = &rec_files_groups[0];

    let part = find_parts(&rec_files_groups, module_dir, environment, overwrite)?;

    let upload_method = Some("manifest".to_string()); // TODO cleanup this whole property in the specs

    Ok(v2::Okh {
        okhv: v2::OKHV.to_owned(),
        upload_method,
        name: pv(environment, Key::Name)?,
        organisation: None,
        repo: pv(environment, Key::RepoWebUrl).map(|url_str| Url::parse(&url_str))??,
        version: pv(environment, Key::Version)?,
        license,
        readme,
        contribution_guide,
        licensor,
        image: image.clone(),
        documentation_language,
        bom,
        tsdc: None,
        attestation: vec![],
        standard_compliance: vec![],
        cpc_patent_class: None,
        release: None, // TODO Fetch from GH/GL API NOTE This is not the version, but the URL to the release page/archive
        timestamp,     // TODO Which format should this have?
        fork_of: None, // TODO Check GH/GL API (most/all rust libraries for these APIs seem not to support this, so might have to be done by manually querying the APIs)
        function: None, // TODO Get these from GH/GL API Labels/Tags, maybe? -> checked already; neither GH nor GL APIs offer to fetch thee; would have to be parsed from the HTML of the web-view
        documentation_readiness_level: None, // TODO
        technology_readiness_level: None, // TODO
        user_manual: None, // TODO How? -> "*(manual|user)*.md"
        manufacturing_instructions: vec![], // TODO How? -> "*(assembly|guide)*.md"
        software: vec![], // TODO How? -> directories called "*(software|firmware)*" maybe?
        standard: vec![], // TODO How?
        source: vec![], // TODO -> scan for known source files extensions (but not inside parts folders)
        export: vec![], // TODO -> scan for known export files extensions (but not inside parts folders)
        auxiliary: vec![], // TODO -> scan for known aux. files extensions (but not inside parts folders)
        part,
    })
}

pub fn okh_losh_toml_part(
    repo_root: &Path,
    sub_part: &RelativePath,
    environment: Option<&Environment>,
    overwrite: bool,
) -> Res<()> {
    let owned_env = if environment.is_some() {
        None
    } else {
        println!(
            "XXX running projvar in '{}' - '{}' ...",
            repo_root.display(),
            sub_part
        );
        Some(run_projvar(repo_root)?)
    };
    let environment =
        environment.map_or_else(|| owned_env.as_ref().unwrap(), |environment| environment);
    let module_dir = sub_part.to_path(repo_root);
    // println!("XXX ran projvar in '{}' - '{}'.", repo_root.display(), sub_part);

    let okh_losh = generate_data(&module_dir, environment, overwrite)?;

    let manifest_file = module_dir.join(v2::MANIFEST_FILE_NAME);
    if !manifest_file.exists() || overwrite {
        log::debug!("Writing to TOML file ...");

        // construct the REUSE/SPDX license header
        let git_user_name = String::from_utf8(
            Command::new("git")
                .arg("config")
                .arg("user.name")
                .output()?
                .stdout,
        )?;
        let git_user_name = git_user_name.trim_end();
        let git_user_email = String::from_utf8(
            Command::new("git")
                .arg("config")
                .arg("user.email")
                .output()?
                .stdout,
        )?;
        let git_user_email = git_user_email.trim_end();
        let now: DateTime<Utc> = Utc::now();
        let header = format!(
            "# SPDX-FileCopyrightText: {} {} <{}>
#
# SPDX-License-Identifier: CC0-1.0",
            now.year(),
            git_user_name,
            git_user_email
        );

        let content = okh_losh.to_toml()?;

        fs::write(manifest_file, format!("{header}\n\n{content}"))?;
    } else {
        log::warn!(
            "Skipped writing '{}': File already exists. See `--overwrite`.",
            manifest_file.display()
        );
    }

    Ok(())
}

fn is_git_submodule(sub_part_path: &Path) -> bool {
    git2::Repository::open(sub_part_path).is_ok()
}

pub fn okh_losh_toml(proj_root: &Path, overwrite: bool) -> Res<()> {
    okh_losh_toml_part(proj_root, &RelativePathBuf::new(), None, overwrite)
}
