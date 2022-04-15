// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod language;

use chrono::Datelike;
use projvar::var::Key;

use chrono::{DateTime, Utc};
use std::process::Command;
use std::{fs, path::Path};

use crate::formats::v2;
use crate::formats::SerError;
use crate::macros::rgx;
use crate::{dir, license};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to find a value for projvar key '{0:?}'.")]
    PVKeyNotFound(Key),

    #[error("Failed to read or write, probably from/to the file-system.")]
    Io(#[from] std::io::Error),

    #[error("Misc error.")]
    General(#[from] Box<dyn std::error::Error>),

    #[error("Failed to serialize to TOML/YAML/similar.")]
    Serialization(#[from] SerError),

    #[error("Failed to initialize a git repo.")]
    Git2RepoInit(#[from] git2::Error),

    #[error("Failed to convert string to UTF-8.")]
    NonUtf8(#[from] std::string::FromUtf8Error),
}

type Res<O> = Result<O, Error>;
type OString = Option<String>;

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

pub fn find_root_files(proj_root: &Path) -> (OString, OString, OString) {
    let root_file_filters = &[
        rgx!(r#"README.*(\.(md|markdown))?"#),
        rgx!(r#"[Bb](ill)?[-_]?[Oo](f)?[-_]?[Mm](aterials)?"#),
        rgx!(r#"CONTRIBUTI(NG|ON)?(\.(md|markdown))?"#),
    ];
    let found_files = dir::scan(proj_root, false, root_file_filters, Path::file_name);
    let mut single_found_files = found_files.iter().map(|fnds| {
        let mut sorted = fnds.clone();
        sorted.sort_by_key(|pth| pth.as_os_str().len());
        sorted.get(0).map(|p| p.display().to_string())
    });

    (
        single_found_files.next().unwrap(),
        single_found_files.next().unwrap(),
        single_found_files.next().unwrap(),
    )
}

pub fn find_rec_files(proj_root: &Path) -> Vec<String> {
    let file_ext_filters = &[rgx!(r#"^(png|jpg|jpeg|gif|svg)$"#), rgx!(r#"^()$"#)]; // TODO Write the second filter and use it
    let found_rec_files = dir::scan(proj_root, true, file_ext_filters, Path::extension);
    let mut found_rec_files = found_rec_files.iter().map(|fnds| {
        let mut sorted = fnds.clone();
        sorted.sort_by_key(|pth| pth.as_os_str().len());
        sorted
    });
    found_rec_files
        .next()
        .unwrap()
        .iter()
        .map(|p| p.display().to_string())
        .collect()
}

fn first_commit(repo: &git2::Repository) -> Res<git2::Commit<'_>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;
    Ok(repo.find_commit(revwalk.next().unwrap()?)?)
}

pub fn okh_losh_toml(proj_root: &Path, overwrite: bool) -> Res<()> {
    let environment = run_projvar(proj_root)?;

    let license = license::ensure_spdx_license_id(&pv(&environment, Key::License)?);

    let documentation_language = language::identify(&fs::read_to_string("README.md")?); // TODO HACK look for README* instead!

    let version = pv(&environment, Key::Version)?;
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
    let repo = git2::Repository::open(proj_root)?;
    let first_commit = first_commit(&repo)?;
    // println!("first commit msg: {}", first_commit.message().unwrap());
    let git_author = first_commit.author().to_string();
    let licensor = git_author;

    let (readme, bom, contribution_guide) = find_root_files(proj_root);

    let image = find_rec_files(proj_root);

    let okh_losh = v2::Okh {
        okhv: v2::OKHV.to_owned(),
        name: pv(&environment, Key::Name)?,
        organisation: None,
        repo: pv(&environment, Key::RepoWebUrl)?,
        version: pv(&environment, Key::Version)?,
        license,
        readme,
        contribution_guide,
        licensor,
        image,
        documentation_language,
        bom,
        tsdc: None,
        attestation: vec![],
        standard_compliance: vec![],
        cpc_patent_class: None,
        release: None, // TODO Fetch from GH/GL API NOTE This is not the version, but the URL to the release page/archive
        timestamp,     // TODO Which format should this have?
        fork_of: None, // TODO Check GH/GL API (most/all rust libraries for these APIs seem not to support this, so might have to be done by manually querying the APIs)
        function: None, // TODO Get these from GH/GL API Labels/Tags, maybe?
        documentation_readiness_level: None, // TODO
        technology_readiness_level: None, // TODO
        user_manual: None, // TODO How? -> "*(manual|user)*.md"
        manufacturing_instructions: vec![], // TODO How? -> "*(assembly|guide)*.md"
        software: vec![], // TODO How? -> directories called "*(software|firmware)*" maybe?
        standard: vec![], // TODO How?
        source: vec![], // TODO -> scan for known source files extensions (but not inside parts forlders)
        export: vec![], // TODO -> scan for known export files extensions (but not inside parts forlders)
        auxiliary: vec![], // TODO -> scan for known aux. files extensions (but not inside parts forlders)
        part: vec![],      // TODO -> scan for the three file types above, but insode sub-foldersq
    };

    let manifest_file = proj_root.join(v2::MANIFEST_FILE_NAME);
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
        let header = format!(
            "# SPDX-FileCopyrightText: {} {} <{}>
#
# SPDX-License-Identifier: CC0-1.0",
            now.year(),
            git_user_name,
            git_user_email
        );

        let content = okh_losh.to_toml()?;

        fs::write(manifest_file, format!("{}\n\n{}", header, content))?;
    } else {
        log::warn!(
            "Skipped writing '{}': File already exists. See `--overwrite`.",
            manifest_file.display()
        );
    }

    Ok(())
}
