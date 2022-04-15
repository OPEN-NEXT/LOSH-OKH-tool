// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

// mod data;

use std::{borrow::Cow, fs, /*io::Read, */path::Path};

use regex::Regex;
use serde::Deserialize;

// use crate::macros::rgx;
use crate::codify::Codify;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Failed to read data to be parsed (e.g. from a file)")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse CSV: {0}")]
    Csv(#[from] csv::Error),
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum OptBool {
    False,
    True,
    #[serde(rename = "-")]
    None,
}

impl OptBool {
    pub fn to_native(&self) -> Option<bool> {
        match self {
            Self::False => Some(false),
            Self::True => Some(true),
            Self::None => None,
        }
    }

    pub fn init_code(&self) -> &str {
        match self {
            Self::False => "Some(false)",
            Self::True => "Some(true)",
            Self::None => "None",
        }
    }
}

// #[derive(Eq)]
pub struct RegexEq(pub Regex);

impl PartialEq for RegexEq {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for RegexEq {
}

impl core::hash::Hash for RegexEq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}

impl std::ops::Deref for RegexEq {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// #[derive(PartialEq, Eq)]
// #[derive(Hash)]
pub struct Rec<'a> {
    pub path: &'a str,
    pub fixed: bool,
    pub source: bool,
    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // module: Option<bool>,
    pub module: bool,
    pub arbitrary_content: Option<bool>,
    pub indicativeness: f32,
    pub regex: RegexEq,
    pub description: &'a str,
}

impl PartialEq for Rec<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.regex == other.regex
    }
}

impl Eq for Rec<'_> {
}

impl core::hash::Hash for Rec<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.regex.hash(state);
    }
}

// We don't need to derive `Debug` (which doesn't require Serde), but it's a
// good habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as
// the fields in the CSV data!
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub path: String,
    pub fixed: bool,
    pub source: bool,
    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // module: Option<bool>,
    pub module: bool,
    pub arbitrary_content: OptBool,
    pub indicativeness: f32,
    #[serde(with = "serde_regex")]
    pub regex: Regex,
    pub description: String,
}
    // #[cfg(feature = "v1_non_losh")]
    // #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub manifest_author: Option<Person>,
    // #[serde(default)]
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    // pub operating_instructions: Vec<Document>,

impl Codify for Record {
    fn init_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!(r##"dir_std::format::Rec {{
            path: r#"{}"#,
            fixed: {},
            source: {},
            module: {},
            arbitrary_content: {},
            indicativeness: {} as f32,
            regex: dir_std::format::RegexEq(Regex::new(r#"{}"#).unwrap()),
            description: r#"{}"#,
        }}"##,
            self.path,
            self.fixed,
            self.source,
            self.module,
            self.arbitrary_content.init_code(),
            self.indicativeness,
            self.regex.to_string(),
            self.description,
        ))
    }
}

pub struct DirStd {
    // name: &'static str,
    // combined_likelihood: f32,
    pub records: Vec<Rec<'static>>,
}

pub struct DirStandard {
    // name: &'static str,
    // combined_likelihood: f32,
    pub records: Vec<Record>,
}

impl Codify for DirStandard {
    fn init_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!(r##"dir_std::format::DirStd {{
            records: {},
        }}"##,
            self.records.init_code(),
        ))

//     writeln!(&mut dir_stds_out, r##"dir_std::format::Rec {{"##)?;

//     let dir_stds = read_dir_stds()?;
//     for (name, records) in dir_stds {
//         writeln!(&mut dir_stds_out, r##"records: vec!("##, name)?;
//         for record in records {
//             writeln!(&mut dir_stds_out, "{},
//             ", record.init_code())?;
//         }
//         writeln!(&mut dir_stds_out, r##"));"##)?;
//     }

//     writeln!(&mut dir_stds_out, r##"stds
// }});
// "##)?;
    }
}

impl DirStandard {
    pub fn from_csv_reader<R: std::io::Read>(rdr: &mut csv::Reader<R>) -> Result<DirStandard, ParseError> {
        let mut records_raw = vec!();
        // with this we ensure, that all the records `indicativeness` values
        // add up to ~= 1.0
        let mut indicativeness_sum = 0 as f32;
        for result in rdr.deserialize() {
            let record: Record = result?;
            indicativeness_sum += record.indicativeness;
            records_raw.push(record);
            // println!("{:?}", record);
            // Try this if you don't like each record smushed on one line:
            // println!("{:#?}", record);
        }
        let mut records = vec!();
        for mut record in records_raw.into_iter() {
            record.indicativeness = record.indicativeness / indicativeness_sum;
            records.push(record);
        }

        Ok(DirStandard { records })
    }
    
    // pub fn from_csv_read(csv_input: &mut dyn Read) -> Result<Vec<Record>, ParseError> {
    //     let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(csv_input);
    //     Self::from_csv_reader(&mut rdr)
    // }

    // pub fn from_csv(csv_str: &str) -> Result<Vec<Record>, ParseError> {
    //     // log::debug!("Parsing CSV to Records ...");
    //     // let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());

    //     // let mut records = vec!();
    //     // for result in rdr.deserialize() {
    //     //     let record: Record = result?;
    //     //     records.push(record);
    //     //     // println!("{:?}", record);
    //     //     // Try this if you don't like each record smushed on one line:
    //     //     // println!("{:#?}", record);
    //     // }

    //     let mut csv_bytes = csv_str.as_bytes_mut();
    //     Self::from_csv_reader(csv_bytes)
    // }

    pub fn from_csv_file(csv_file: &Path) -> Result<DirStandard, ParseError> {
        // log::debug!("Reading CSV file to string ...");
        // let csv_str = fs::read_to_string(csv_file)?;

        // Self::from_csv(&csv_str)

        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(fs::File::open(csv_file)?);
        Self::from_csv_reader(&mut rdr)
    }

    // pub fn to_yaml(&self) -> Result<String, SerError> {
    //     log::debug!("Serializing v1 to YAML ...");
    //     Ok(serde_yaml::to_string(self)?)
    // }

    // pub fn to_yaml_file(&self, yaml_file: &Path) -> Result<(), SerError> {
    //     let serialized = self.to_yaml()?;
    //     log::debug!("Writing v1 to YAML file ...");
    //     fs::write(yaml_file, serialized)?;
    //     Ok(())
    // }

    // pub fn ext_matcher() -> &'static Regex {
    //     rgx!(r#"(^|\.)[cC][sS][vV]$"#)
    // }
}
