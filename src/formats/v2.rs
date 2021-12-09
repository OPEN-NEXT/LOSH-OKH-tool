// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{fs, path::Path};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    macros::rgx,
    oxrl::{Odrl, Otrl},
};

use super::{ParseError, SerError};

pub type DSString = String;

pub const OKHV: &str = "OKH-LOSHv1.0";
pub const MANIFEST_FILE_NAME: &str = "okh.toml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Software {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<DSString>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubMosh {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_file: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<DSString>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Okh {
    pub okhv: DSString,

    pub name: DSString,

    pub repo: DSString,

    pub version: DSString,

    pub license: DSString,

    pub licensor: DSString,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_guide: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_language: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attestation: Vec<DSString>,

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
    pub bom: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<DSString>,

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
    pub user_manual: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_instructions: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub software: Vec<Software>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub standard: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auxiliary: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<SubMosh>,
}

impl Okh {
    pub fn from_toml(toml_str: &str) -> Result<Okh, ParseError> {
        log::debug!("Parsing TOML to v2 ...");
        let parsed = toml::from_str::<Okh>(toml_str)?;
        // TODO Add extra sanity checks here, if required
        Ok(parsed)
    }

    pub fn from_toml_file(toml_file: &Path) -> Result<Okh, ParseError> {
        log::debug!("Reading TOML file to string ...");
        let toml_str = fs::read_to_string(toml_file)?;

        Self::from_toml(&toml_str)
    }

    pub fn to_toml(&self) -> Result<String, SerError> {
        log::debug!("Serializing to TOML ...");
        Ok(toml::to_string(self)?)
    }

    pub fn to_toml_file(&self, toml_file: &Path) -> Result<(), SerError> {
        let serialized = self.to_toml()?;
        log::debug!("Writing to TOML file ...");
        fs::write(toml_file, serialized)?;
        Ok(())
    }

    pub fn ext_matcher() -> &'static Regex {
        rgx!(r#"(^|\.)[tT][oO][mM][lL]$"#)
    }
}
