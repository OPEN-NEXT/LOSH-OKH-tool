// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{fs, path::Path};

use regex::Regex;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::macros::rgx;

use super::{Locator, ParseError, SerError};

type DSString = String;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Document {
    #[serde(default)]
    pub title: Option<DSString>,

    #[serde(default)]
    pub path: Option<Locator>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct License {
    /// The license under which the hardware is released
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<DSString>,

    /// The license under which the documentation is released
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<DSString>,

    /// The license under which the software is released
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<DSString>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Person {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<DSString>,

    /// This is only used in the [`create::v1::Okh::contact`] property.
    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub social: Vec<Social>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Standard {
    #[serde(default)]
    pub reference: DSString,

    #[serde(default)]
    pub standard_title: DSString,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct OtherThing {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<Url>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct OtherThingWithLang {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<DSString>,
}

#[allow(clippy::unnecessary_wraps)]
fn default_okh_manifest_version() -> Option<DSString> {
    Some("1.0.0".to_owned())
}

/// Thi srepresents the contents of an OKH v1 manifest file (YAML),
/// and allows us to parse such a file (okh.yml) with serde.
/// It is based on the contents of the quasi-schema/stnadard by OHK themselfs,
/// which can be found here:
/// <https://git.iostud.io/makernet/iop-cdb/-/blob/dev/server/assets/okh.okhdf>
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
// #[serde(default)]
pub struct Okh {
    /// Name of the hardware
    pub title: DSString,

    /// Description of the hardware
    pub description: DSString,

    /// Intended use for the hardware
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_use: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<DSString>,

    /// Link to the project
    pub project_link: Option<Url>,

    /// Link to an image
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<RelativePathBuf>,

    /// Someone has made this hardware
    #[serde(default)] // = false
    pub made: bool,

    /// Someone independent has made this hardware
    #[serde(default)] // = false
    pub made_independently: bool,

    pub license: License,

    /// Who granted the license
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensor: Option<Person>,

    #[serde(default = "default_okh_manifest_version")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub okh_manifest_version: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<DSString>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_author: Option<Person>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_langauge: Option<DSString>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_is_translation: Option<OtherThingWithLang>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<PersonSocial>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<Person>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development_stage: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_home: Option<Url>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_download: Option<Url>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub design_files: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_language: Option<DSString>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_is_translation: Option<OtherThingWithLang>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schematics: Vec<Document>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom: Option<RelativePathBuf>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools_list: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub making_instructions: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_files: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub risk_assessment: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tool_settings: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quality_instructions: Vec<Document>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub operating_instructions: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintenance_instructions: Vec<Document>,

    #[cfg(feature = "v1_non_losh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disposal_instructions: Vec<Document>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub software: Vec<Document>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_safety_notice: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub standards_used: Vec<Standard>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivative_of: Option<OtherThing>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_of: Option<OtherThing>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "sub")] // This is the old, deprecated name
    pub sub_parts: Vec<OtherThing>,
}

impl Okh {
    pub fn main_url(&self) -> Option<&Url> {
        self.project_link
            .as_ref()
            .or(self.documentation_home.as_ref())
    }
}

impl Okh {
    pub fn from_yaml(yaml_str: &str) -> Result<Okh, ParseError> {
        log::debug!("Parsing YAML to v1 ...");
        let parsed = serde_yaml::from_str::<Okh>(yaml_str)?;

        if let (None, None, None) = (
            &parsed.license.documentation,
            &parsed.license.hardware,
            &parsed.license.software,
        ) {
            panic!("at least one of the three licenses (documentation, hardware or software) has to be set");
        }

        Ok(parsed)
    }

    pub fn from_yaml_file(yaml_file: &Path) -> Result<Okh, ParseError> {
        log::debug!("Reading YAML file to string ...");
        let yaml_str = fs::read_to_string(yaml_file)?;

        Self::from_yaml(&yaml_str)
    }

    pub fn to_yaml(&self) -> Result<String, SerError> {
        log::debug!("Serializing v1 to YAML ...");
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn to_yaml_file(&self, yaml_file: &Path) -> Result<(), SerError> {
        let serialized = self.to_yaml()?;
        log::debug!("Writing v1 to YAML file ...");
        fs::write(yaml_file, serialized)?;
        Ok(())
    }

    pub fn ext_matcher() -> &'static Regex {
        rgx!(r#"(^|\.)[yY][aA]?[mM][lL]$"#)
    }
}
