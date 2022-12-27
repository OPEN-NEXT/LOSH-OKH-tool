// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{fs, path::Path};

use regex::Regex;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    macros::rgx,
    oxrl::{Odrl, Otrl},
};

use super::{ParseError, SerError};

pub type DSString = String;

pub const OKHV: &str = "OKH-LOSHv1.0";
pub const MANIFEST_FILE_NAME: &str = "okh.toml";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Software {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<Url>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SubMosh {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsdc: Option<DSString>,

    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub manifest_file: Option<RelativePathBuf>,

    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub repo: Option<Url>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auxiliary: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<SubMosh>,
}

// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
// #[serde(rename_all = "kebab-case")]
// pub struct Meta {
//     pub source: DSString,

// // [__meta]
// // source = "github.com"
// // owner = "KarlK90"
// // repo = "yaemk-split-kb"
// // path = "okh.toml"
//     #[serde(default)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub owner: Option<DSString>,

//     #[serde(default)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub repo: Option<DSString>,

//     #[serde(default)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub path: Option<DSString>,
// }

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Okh {
    pub okhv: DSString,

    pub name: DSString,

    pub repo: Url,

    pub version: DSString,

    pub license: DSString,

    pub licensor: DSString,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_method: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_guide: Option<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_language: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attestation: Vec<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub standard_compliance: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpc_patent_class: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsdc: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom: Option<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_readiness_level: Option<Odrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_readiness_level: Option<Otrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_manual: Option<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_instructions: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub software: Vec<Software>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub standard: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auxiliary: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<SubMosh>,
    // #[serde(default)]
    // #[serde(rename = "__meta")]
    // pub __meta: Meta,
}

impl Okh {
    pub fn from_toml(toml_str: &str) -> Result<Okh, ParseError> {
        log::debug!("Parsing TOML to v2 ...");
        let parsed = toml::from_str::<Okh>(toml_str)?;
        // TODO Add extra sanity checks here, if required
        Ok(parsed)
    }

    pub fn from_toml_file<OP>(toml_file: OP) -> Result<Okh, ParseError>
    where
        OP: AsRef<Path>,
    {
        log::debug!("Reading TOML file to string ...");
        let toml_str = fs::read_to_string(toml_file)?;

        Self::from_toml(&toml_str)
    }

    pub fn to_toml(&self) -> Result<String, SerError> {
        log::debug!("Serializing to TOML ...");
        Ok(toml::to_string(self)?)
    }

    pub fn to_toml_file<OP>(&self, toml_file: OP) -> Result<(), SerError>
    where
        OP: AsRef<Path>,
    {
        let serialized = self.to_toml()?;
        log::debug!("Writing to TOML file ...");
        fs::write(toml_file, serialized)?;
        Ok(())
    }

    pub fn ext_matcher() -> &'static Regex {
        rgx!(r#"(^|\.)[tT][oO][mM][lL]$"#)
    }
}
