// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use crate::formats::v1;
use crate::formats::v2;
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
    if let Ok(repo) = repo {
        let head = repo.head()?;
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

fn shorten_to_repo_url(manifest_url: &str) -> Option<String> {
    let parts: Vec<&str> = manifest_url.rsplit('/').collect();
    let mut parts = parts.iter();
    parts.next().map(|s| s.to_owned().to_owned())
}

fn version(v1: &v1::Okh) -> Result<String, Error> {
    v1.version.as_ref().map_or(
        Err(Error::InsufficientData {
            msg: "'version' is required for OKH LOSH",
        }),
        |version| Ok(version.trim().to_string()),
    )
}

fn repo(v1: &v1::Okh) -> Result<String, Error> {
    v1.documentation_home.as_ref().map_or(
        Err(Error::InsufficientData {
            msg: "OKH v1 'documentation_home' is required to convert to OKH LOSH",
        }),
        |repo| Ok(repo.clone()),
    )
}

fn timestamp(v1: &v1::Okh) -> Option<String> {
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

fn fork_of(v1: &v1::Okh) -> Result<Option<String>, Error> {
    if let Some(parent) = [&v1.derivative_of, &v1.variant_of]
        .iter()
        .find_map(|&val| val.as_ref())
    {
        return Ok(if let Some(manifest) = &parent.manifest {
            shorten_to_repo_url(manifest)
        } else if let Some(web) = &parent.web {
            Some(web.clone())
        } else {
            return Err(Error::InsufficientData { msg: "For 'derivative-of' and 'variant-of', at least one of 'web' and 'manifest' needs to be specified" });
        });
    }
    Ok(None)
}

fn function(v1: &v1::Okh) -> String {
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

fn documentation_readiness_level(v1: &v1::Okh) -> Odrl {
    if v1.made_independently {
        Odrl::Full
    } else {
        Odrl::Started // TODO HACK What should really be here?
    }
}

fn is_prototype(development_stage: &Option<String>) -> bool {
    development_stage
        .as_ref()
        .map(|ds| ds.to_lowercase() == "prototype")
        .unwrap_or_default()
}

fn technology_readiness_level(v1: &v1::Okh) -> Otrl {
    if v1.made_independently || v1.made || is_prototype(&v1.development_stage) {
        Otrl::TechnicalDesign
    } else {
        Otrl::Ideation // TODO HACK What should really be here?
    }
}

fn license(v1: &v1::Okh) -> Result<String, Error> {
    if let Some(lcse) = [&v1.license.hardware, &v1.license.documentation]
        .iter()
        .find_map(|&val| val.as_ref())
    {
        return Ok(license::ensure_spdx_license_id(lcse));
    }
    Err(Error::NoLicense)
}

fn licensor(v1: &v1::Okh) -> Result<String, Error> {
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
                if let Some(name) = name {
                    licensor_str.push_str(name.trim());
                }
                if let Some(email) = email {
                    if !licensor_str.is_empty() {
                        licensor_str.push(' ');
                    }
                    licensor_str.push('<');
                    licensor_str.push_str(email.trim());
                    licensor_str.push('>');
                }
                Ok(licensor_str)
                // Ok(Some(licensor_str))
            }
        },
    )
}

fn software(v1: &v1::Okh) -> Vec<v2::Software> {
    let mut software = Vec::<v2::Software>::new();
    for sw in &v1.software {
        software.push(v2::Software {
            label: sw.title.clone(),
            release: sw.path.clone(),
        });
    }
    software
}

fn standard(v1: &v1::Okh) -> Vec<v2::DSString> {
    let mut standard = Vec::<v2::DSString>::new();
    for strd in &v1.standards_used {
        if !strd.reference.is_empty() {
            standard.push(strd.reference.trim().to_string());
        }
        if !strd.standard_title.is_empty() {
            standard.push(strd.standard_title.trim().to_string());
        }
    }
    standard
}

fn sub_mosh(v1: &v1::Okh) -> Result<Vec<v2::SubMosh>, Error> {
    let mut sub_mosh = Vec::<v2::SubMosh>::new();
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
        sub_mosh.push(v2::SubMosh {
            name: parent.title.clone(),
            manifest_file: parent.manifest.clone(),
            repo,
        });
    }
    Ok(sub_mosh)
}

fn collect_doc_paths(docs: &'_ [v1::Document]) -> impl '_ + Iterator<Item = String> {
    docs.iter().filter_map(|doc| doc.path.clone())
}

fn collect_doc_path(docs: &[v1::Document]) -> Option<String> {
    collect_doc_paths(docs).next() // TODO FIXME This irgnores all but the first document with a path!
}

pub fn convert(v1: v1::Okh) -> Result<v2::Okh, Error> {
    log::debug!("Converting OKH v1 to v2 ...");
    let version = version(&v1)?;
    let repo = repo(&v1)?;
    let organisation = None;
    let readme = None;
    let contribution_guide = None;
    let attestation = vec![];
    let standard_compliance = v1
        .standards_used
        .iter()
        .map(|std| std.standard_title.trim().to_string())
        .collect();
    let cpc_patent_class = None;
    let tsdc = None;
    let image = v1.image.iter().map(String::to_owned).collect();
    let timestamp = timestamp(&v1);
    let fork_of = fork_of(&v1)?;
    let function = Some(function(&v1));
    let documentation_readiness_level = Some(documentation_readiness_level(&v1));
    let technology_readiness_level = Some(technology_readiness_level(&v1));
    let license = license(&v1)?;
    let licensor = licensor(&v1)?;
    let manufacturing_instructions = collect_doc_paths(&v1.making_instructions).collect();
    let user_manual = collect_doc_path(&v1.operating_instructions); // TODO FIXME This irgnores all but the first operating instruction!
    let software = software(&v1);
    let standard = standard(&v1);
    let source = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let export = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let auxiliary = vec![]; // TODO see v1.design_files and v1.schematics, but split into source, export and auxiliary!
    let part = sub_mosh(&v1)?;

    Ok(v2::Okh {
        okhv: v2::OKHV.to_owned(),
        name: v1.title.trim().to_string(),
        organisation,
        readme,
        contribution_guide,
        attestation,
        standard_compliance,
        cpc_patent_class,
        tsdc,
        version,
        image,
        repo,
        documentation_language: v1.documentation_language.map(|s| s.trim().to_string()),
        bom: v1.bom,
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
    })
}

pub fn convert_file(yaml_file: &Path, toml_file: &Path) -> Result<(), Error> {
    log::info!("OKH v1 (YAML) input file:  {}", yaml_file.display());
    log::info!("OKH v2 (TOML) output file: {}", toml_file.display());

    let v1 = v1::Okh::from_yaml_file(yaml_file)?;
    let v2 = convert(v1)?;
    v2.to_toml_file(toml_file)?;

    log::info!("done.");
    Ok(())
}
