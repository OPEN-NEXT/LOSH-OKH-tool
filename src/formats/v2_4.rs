// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// include!(concat!(env!("OUT_DIR"), "/okh_model_v_2_4.rs"));

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
// pub type Float = f64;
pub type Float = DSString;
pub type RelativePathOrUrl = DSString;

pub const OKHV: &str = "2.4";
pub const MANIFEST_FILE_NAME: &str = "okh.toml";

// pub fn bla() {
//     Manifest::deserialize(deserializer)
// }

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Agent {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub person: ::std::option::Option<Person>,

    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub organization: ::std::option::Option<Organization>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Person {
    /// E-Mail of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<DSString>,
    /// Full name of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DSString>,
    /// Home-page of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<DSString>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Dimensions {
    pub width: Float,

    pub depth: Float,
    
    pub height: Float,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RdfNamespace {
    pub namespace: DSString,

    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prefix: ::std::option::Option<DSString>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Organization {
    /// E-Mail of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<DSString>,
    /// Full name of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<DSString>,
    /// Home-page of the person
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<Url>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ImageObject {
    pub location: RelativePathOrUrl,

    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub depicts: ::std::option::Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<DSString>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum Image {
    Location(RelativePathOrUrl),
    Object(ImageObject),
}

impl Default for Image {
    fn default() -> Self {
        Self::Location(Default::default())
    }
}

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
pub struct Part {

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auxiliary: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<Image>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<Part>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsdc: Option<DSString>,
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
    /**reference to one or multiple valid attestation(s) that the documentation is complete and fully qualifies as open source hardware;\
issuing conformity assessment bodies according to DIN SPEC 3105-2:\
- [Open Hardware Observatory](https://en.oho.wiki/wiki/Request_certification_for_your_project)\
- [Open Source Ecology Germany](https://gitlab.opensourceecology.de/verein/projekte/cab/CAB)\
- [OSHWA certification programme](https://certification.oshwa.org/)*/
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attestation: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auxiliary: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom: Option<RelativePathOrUrl>,

    ///repo-relative path to the contribution guide
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_guide: Option<RelativePathOrUrl>,

    /**patent class identifier of the Cooperative Patent Classification that describes best the field of technology of the OSH module.\
Get it from here: <https://worldwide.espacenet.com/classification>*/
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpc_patent_class: Option<DSString>,

    ///IETF BCP 47 language tag(s) for the language(s) in which the documentation is written
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub documentation_language: Vec<DSString>,

    ///ODRL-ID representing the development stage of the documentation; get it from: <https://w3id.org/oseg/ont/otrl>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_readiness_level: Option<Odrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub export: Vec<RelativePathOrUrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<Url>,

    /**functional description, e.g. what it actually does, what problem it solves, for whom, under which conditions etc.\
So if you wish that someone finds & uses your OSH specifically e.g. for COVID-19-crisis response, include relevant keywords in this field*/
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<DSString>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<Image>,

    /**An SPDX license _expression
(see: <https://spdx.github.io/spdx-spec/v2-draft/SPDX-license-expressions/>),
usually a single SPDX license ID
(see complete list: <https://spdx.org/licenses/>),
or a combination of those,
combined with AND or OR.
If your license is not SPDX registered (yet),
use a custom string prefixed with 'LicenseRef-',
for example 'LicenseRef-MyVeryOwnLicense-1.3';
please use the 'SPDX identifier' from
<https://scancode-licensedb.aboutcode.org/>,
if your license is there but not SPDX registered.
Use 'LicenseRef-NOASSERTION' if no license is specified,
'LicenseRef-NONE' if no license is specified
(which usually means: all rights reserved),
or 'LicenseRef-AllRightsReserved' or similar
for projects clearly indicating that they are proprietary.*/
    pub license: DSString,

    ///organization/individual behind the hardware design (holder of intellectual property)
    pub licensor: Agent,

    ///URL or repo-relative path to manufacturing instructions; multiple inputs possible (with one entry each)
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_instructions: Vec<RelativePathBuf>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass: Option<Float>,

    ///Working title of the OSH component
    pub name: DSString,

    ///version of OKH specification the OSH projects metadata follows (different version → different data fields in this file)
    pub okhv: DSString,

    ///organization of the licensor or that represents (some of) the contributors of to project
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub organization: Vec<Organization>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_dimensions: Option<Dimensions>,

    ///physical component(s) of this open source hardware module, for which technical documentation (design files etc.) is available under a free/open license
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<Part>,

    ///The DOI(s) or web URL(s) of one or multiple associated publication(s)
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publication: Vec<DSString>,

    ///absolute HTTP IRI (ending int '/' or '#') representing the RDF namespace of the triples generated from the manifest, and optionally the name of the prefix
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf: Option<RdfNamespace>,

    ///repo-relative path (or absolute HTTP(S) URL) to to the corresponding ReadMe, which is the (human) entry-point into (the sources of) an OSH project
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub readme: Vec<RelativePathOrUrl>,

    ///URL or repo local path to the release package of this version of the OSH module
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<Url>,

    /**URL to the (human browsable) place where development happens;
typically a (git) repository or Wiki page.
Following this link, people should be able to contribute to the development:
reporting issues, suggesting changes, connecting to the team etc.*/
    pub repo: Url,

    // pub schema: ::std::option::Option<ManifestSchema>,

    ///associated software package(s) used to operate this piece of open source hardware
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub software: Vec<Software>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<RelativePathOrUrl>,

    /**document-number of the official standard the OSH module complies;\
multiple inputs possible (with one entry each)*/
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub standard_compliance: Vec<DSString>,

    ///OTRL-ID representing the development stage of the OSH module; get it from: <https://w3id.org/oseg/ont/otrl>
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_readiness_level: Option<Otrl>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsdc: Option<DSString>,

    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub timestamp: Option<DSString>,

    /// URL or repo-relative path to user manual
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_manual: Option<RelativePathOrUrl>,

    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub upload_method: Option<DSString>,

    /// version of this Module, preferably following the [semantic versioning-scheme v2.0.0](https://semver.org/#semantic-versioning-200)
    pub version: DSString,
}

impl Okh {
    pub fn from_toml(toml_str: &str) -> Result<Self, ParseError> {
        log::debug!("Parsing TOML to v2 ...");
        let parsed = toml::from_str::<Self>(toml_str)?;
        // TODO Add extra sanity checks here, if required
        Ok(parsed)
    }

    pub fn from_toml_file<OP>(toml_file: OP) -> Result<Self, ParseError>
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
        rgx!(r"(^|\.)[tT][oO][mM][lL]$")
    }

    pub fn file_matcher() -> &'static Regex {
        rgx!(r"okh\.[tT][oO][mM][lL]$")
    }
}
