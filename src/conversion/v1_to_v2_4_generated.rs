// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use relative_path::RelativePathBuf;
use url::Url;

use crate::formats::v2_4::Bom;
use crate::formats::v2_4::StringMulti;
use crate::formats::v2_4::WebUrl;
use crate::formats::Locator;
use crate::formats::v1 as v_old;
use crate::formats::v2_4 as v_new;
use crate::license;
use crate::oxrl::Odrl;
use crate::oxrl::Otrl;

use super::Error;

/// Returns the commit-time (not author-time)
/// of the last commit in the currently checked out history (=> HEAD)
///
/// # Errors
///
/// If some git-related magic goes south.
// fn git_commit_date(date_format: &str) -> BoxResult<Option<String>> {
fn git_commit_date() -> Result<Option<i64>, Error> {
    let repo = git2::Repository::open(".");
    if let Ok(repo_val) = repo {
        let head = repo_val.head()?;
        let commit_time_git2 = head.peel_to_commit()?.time();
        // let commit_time_chrono = DateTime::<Utc>::from_utc(
        //     NaiveDateTime::from_timestamp(commit_time_git2.seconds(), 0),
        //     Utc,
        // );
        // Ok(Some(commit_time_chrono.format(date_format).to_string()))
        // date.fromtimestamp(repo.head.ref.commit.committed_date).strftime(date_format)
        Ok(Some(commit_time_git2.seconds()))
    } else {
        Ok(None)
    }
}

fn shorten_to_repo_url(manifest_url: &Url) -> Option<Url> {
    let repo_path = RelativePathBuf::from(manifest_url.path());
    let mut repo_url = manifest_url.clone();
    repo_path.parent().map(|parent_path| {
        repo_url.set_path(parent_path.as_ref());
        repo_url
    })
}

fn version(v1: &v_old::Okh) -> Result<String, Error> {
    v1.version.as_ref().map_or(
        Err(Error::InsufficientData {
            msg: "'version' is required for OKH LOSH",
        }),
        |version| Ok(version.trim().to_string()),
    )
}

fn repo(v1: &v_old::Okh) -> Result<Url, Error> {
    v1.documentation_home
        .as_ref()
        .or(v1.project_link.as_ref())
        .map_or(
        Err(Error::InsufficientData {
            msg: "OKH v1 'documentation_home' or 'project_link' is required to convert to OKH LOSH",
        }),
        |repo| Ok(repo.clone()),
    )
}

fn timestamp(v1: &v_old::Okh) -> Option<String> {
    if let Ok(Some(commit_date)) = git_commit_date() {
        Some(commit_date.to_string())
    } else if v1.date_updated.is_some() {
        v1.date_updated.clone()
    } else if v1.date_created.is_some() {
        v1.date_created.clone()
    } else {
        None
    }
}

fn fork_of(v1: &v_old::Okh) -> Result<Option<Url>, Error> {
    if let Some(parent) = [&v1.derivative_of, &v1.variant_of]
        .iter()
        .find_map(|&val| val.as_ref())
    {
        return Ok(if let Some(manifest) = &parent.manifest {
            shorten_to_repo_url(manifest)
        } else if let Some(web) = &parent.web {
            Some(web.clone())
        } else {
            return Err(Error::InsufficientData {
                msg: "For 'derivative-of' and 'variant-of', at least one of 'web' and 'manifest' needs to be specified",
            });
        });
    }
    Ok(None)
}

fn function(v1: &v_old::Okh) -> String {
    let mut function = v1.description.clone();
    if let Some(intended_use) = &v1.intended_use {
        if !function.is_empty() {
            function.push('\n');
        }
        function.push_str(intended_use.trim());
    }
    if let Some(health_safety_notice) = &v1.health_safety_notice {
        if !function.is_empty() {
            function.push('\n');
        }
        function.push_str(health_safety_notice.trim());
    }
    function
}

const fn documentation_readiness_level(v1: &v_old::Okh) -> Odrl {
    if v1.made_independently {
        Odrl::Full
    } else {
        Odrl::Started // TODO HACK What should really be here?
    }
}

fn is_prototype(development_stage: Option<&String>) -> bool {
    development_stage
        .as_ref()
        .is_some_and(|ds| ds.to_lowercase() == "prototype")
}

fn technology_readiness_level(v1: &v_old::Okh) -> Otrl {
    if v1.made_independently || v1.made || is_prototype(v1.development_stage.as_ref()) {
        Otrl::Development
    } else {
        Otrl::Ideation // TODO HACK What should really be here?
    }
}

fn license(v1: &v_old::Okh) -> Result<String, Error> {
    if let Some(lcse) = [&v1.license.hardware, &v1.license.documentation]
        .iter()
        .find_map(|&val| val.as_ref())
    {
        return Ok(license::ensure_spdx_license_id(lcse));
    }
    Err(Error::NoLicense)
}

fn licensor(v1: &v_old::Okh) -> Result<String, Error> {
    v1.licensor.as_ref().map_or(
        Err(Error::InsufficientData {
            msg: "OKH v1 'licensor' is required to convert to OKH LOSH",
        }),
        |licensor| match (licensor.name.as_ref(), licensor.email.as_ref()) {
            (None, None) => Err(Error::InsufficientData {
                msg: "'licensor' must define at least one of 'name' and 'email'",
            }),
            (name, email) => {
                let mut licensor_str = String::new();
                if let Some(name_val) = name {
                    licensor_str.push_str(name_val.trim());
                }
                if let Some(email_val) = email {
                    if !licensor_str.is_empty() {
                        licensor_str.push(' ');
                    }
                    licensor_str.push('<');
                    licensor_str.push_str(email_val.trim());
                    licensor_str.push('>');
                }
                Ok(licensor_str)
                // Ok(Some(licensor_str))
            }
        },
    )
}

fn convert_locator(old_locator: &Locator) -> Option<v_new::RelPathOrWebUrl> {
    let mut url = None;
    let mut path = None;
    match old_locator {
        Locator::Url(old_url) => url = Some(old_url.as_str().to_owned().into()),
        Locator::Path(old_path) => path = Some(old_path.as_str().to_owned().into()),
    }
    Some(v_new::RelPathOrWebUrl {
        subtype_0: url,
        subtype_1: path,
    })
}

fn software(v1: &v_old::Okh) -> Vec<v_new::Software> {
    let mut software = Vec::<v_new::Software>::new();
    for sw in &v1.software {
        if let Some(sw_path) = sw
                .path
                .as_ref() {
            if let Some(new_locator) = convert_locator(sw_path) {
                software.push(v_new::Software {
                    // label: sw.title.clone(),
                    installation_guide: None,
                    release: new_locator,
                });
            }
                }
    }
    software
}

fn standard(v1: &v_old::Okh) -> Vec<v_new::DSString> {
    let mut standards = Vec::<v_new::DSString>::new();
    for standard in &v1.standards_used {
        if !standard.reference.is_empty() {
            standards.push(standard.reference.trim().to_string());
        }
        if !standard.standard_title.is_empty() {
            standards.push(standard.standard_title.trim().to_string());
        }
    }
    standards
}

fn sub_mosh(v1: &v_old::Okh) -> Result<Vec<v_new::Part>, Error> {
    let mut sub_mosh = Vec::<v_new::Part>::new();
    for parent in &v1.sub_parts {
        let repo = if let Some(manifest) = &parent.manifest {
            shorten_to_repo_url(manifest)
        } else if let Some(web) = &parent.web {
            Some(web.clone())
        } else {
            return Err(Error::InsufficientData {
                msg: "For 'sub(-part)', at least one of 'web' and 'manifest' needs to be specified",
            });
        };
        let name = parent.title.clone().map(|s| s.trim().to_string()).ok_or(Error::InsufficientData { msg: "Missing part title (OKH v1)" })?.into();
        let main_url = v1.main_url().map(Url::to_string);
        let manifest_file = parent.manifest.clone().and_then(|mf_url| {
            main_url.and_then(|main_url_val| {
                let mf_url_str = mf_url.to_string();
                mf_url_str
                    .strip_prefix(&main_url_val)
                    .map(RelativePathBuf::from)
            })
        });
        let image = None;
        let tsdc = None;
        let source = None; // TODO This or export is required
        let export = None; // TODO This or source is required
        let auxiliary = None;
        let mass = None;
        let outer_dimensions = None;
        // let part = vec![];
        if false {
            // TODO As neither source nor export is set, we have to skip this for now, as otherwise we'd generate an invalid manifest
            sub_mosh.push(v_new::Part {
                name,
                // manifest_file,
                // repo,
                image,
                tsdc,
                source,
                export,
                auxiliary,
                mass,
                outer_dimensions,
            });
        }
    }
    Ok(sub_mosh)
}

// fn data_source(v1: &v_old::Okh) -> v_new::DSString {
//     todo!()
// }

fn collect_doc_paths(docs: &'_ [v_old::Document]) -> impl '_ + Iterator<Item = RelativePathBuf> {
    docs.iter().filter_map(|doc| match &doc.path {
        Some(Locator::Path(path)) => Some(path.clone()),
        _ => None,
    })
}

fn collect_doc_to_new_paths(docs: &'_ [v_old::Document]) -> Option<v_new::RelPathOrWebUrlMulti> {
    let rel_paths = collect_doc_paths(docs);
}

fn collect_doc_path(docs: &[v_old::Document]) -> Option<RelativePathBuf> {
    collect_doc_paths(docs).next() // TODO FIXME This ignores all but the first document with a path!
}


fn to_string_multi(strs: Vec<String>) -> Option<StringMulti> {
    if strs.is_empty() {
        None
    } else if strs.len() == 1 {
        Some(StringMulti::Variant0(strs.into_iter().next().unwrap()))
    } else {
        Some(StringMulti::Variant1(strs))
    }
}

pub fn convert(v1: v_old::Okh) -> Result<v_new::Okh, Error> {
    log::debug!("Converting OKH v1 to v2 ...");
    let version = version(&v1)?;
    let repo = Some(WebUrl(repo(&v1)?.to_string()));
    let organisation = None;
    let readme = None;
    let contribution_guide = None;
    let attestation = vec![];
    let standard_compliance = to_string_multi(v1
        .standards_used
        .iter()
        .map(|std| std.standard_title.trim().to_string())
        .collect());
    let cpc_patent_class = None;
    let tsdc = None;
    let timestamp = timestamp(&v1);
    let fork_of = fork_of(&v1)?.map(|url| WebUrl(url.to_string()));
    let function = Some(function(&v1));
    let documentation_readiness_level = Some(documentation_readiness_level(&v1));
    let technology_readiness_level = Some(technology_readiness_level(&v1));
    let license = license(&v1)?;
    let licensor = licensor(&v1)?;
    let manufacturing_instructions = collect_doc_to_new_paths(&v1.making_instructions);
    let user_manual = collect_doc_path(&v1.operating_instructions); // TODO FIXME This ignores all but the first operating instruction!
    let software = software(&v1);
    let standard = standard(&v1);
    let source = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let export = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let auxiliary = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let part = sub_mosh(&v1)?;
    let image = v1.image.into_iter().collect();
    let upload_method = Some("manifest-script".to_string()); // TODO cleanup this whole property in the specs
    // let __meta = v_new::Meta {
    //     source: data_source(&v1),
    //     owner: None,
    //     repo: None,
    //     path: None,
    // };

    Ok(v_new::Okh {
        okhv: v_new::OKHV.to_owned(),
        upload_method,
        name: v1.title.trim().to_string(),
        organisation,
        readme,
        contribution_guide,
        attestation: collect_doc_to_new_paths(&attestation),
        standard_compliance,
        cpc_patent_class,
        tsdc,
        version: Some(version),
        image,
        repo,
        documentation_language: v1.documentation_language.map(|s| s.trim().to_string()),
        bom: v1.bom.map(|rel_path| Bom(rel_path.to_string())),
        release: v1.archive_download,
        timestamp,
        fork_of,
        function,
        documentation_readiness_level,
        technology_readiness_level,
        license,
        licensor,
        manufacturing_instructions,
        user_manual,
        software,
        standard,
        source,
        export,
        auxiliary,
        part,
        // __meta,
    })
}

pub fn convert_file<IP, OP>(yaml_file: IP, toml_file: OP) -> Result<(), Error>
where
    IP: AsRef<Path>,
    OP: AsRef<Path>,
{
    log::info!(
        "OKH v1 (YAML) input file:  {}",
        yaml_file.as_ref().display()
    );
    log::info!(
        "OKH v2 (TOML) output file: {}",
        toml_file.as_ref().display()
    );

    let v1 = v_old::Okh::from_yaml_file(yaml_file)?;
    let v2 = convert(v1)?;
    v2.to_toml_file(toml_file)?;

    log::info!("done.");
    Ok(())
}
