// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod data;
mod format;
mod identifier;

// use std::{fs, io::{BufReader, Read}, path::Path};

// use regex::Regex;
// use serde::Deserialize;

// // use crate::macros::rgx;

// #[derive(thiserror::Error, Debug)]
// pub enum ParseError {
//     #[error("Failed to read data to be parsed (e.g. from a file)")]
//     IO(#[from] std::io::Error),

//     #[error("Failed to parse CSV: {0}")]
//     Csv(#[from] csv::Error),
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum OptBool {
//     False,
//     True,
//     #[serde(rename = "-")]
//     None,
// }

// // We don't need to derive `Debug` (which doesn't require Serde), but it's a
// // good habit to do it for all your types.
// //
// // Notice that the field names in this struct are NOT in the same order as
// // the fields in the CSV data!
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Record {
//     path: String,
//     fixed: bool,
//     source: bool,
//     // #[serde(default)]
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // module: Option<bool>,
//     module: bool,
//     arbitrary_content: OptBool,
//     indicativeness: f32,
//     #[serde(with = "serde_regex")]
//     regex: Regex,
//     description: String,
// }
//     // #[cfg(feature = "v1_non_losh")]
//     // #[serde(default)]
//     // #[serde(skip_serializing_if = "Option::is_none")]
//     // pub manifest_author: Option<Person>,
//     // #[serde(default)]
//     // #[serde(skip_serializing_if = "Vec::is_empty")]
//     // pub operating_instructions: Vec<Document>,

// impl Record {
//     pub fn init_code(&self) -> String {
//         format!(r##"Record {{
//             path: {},
//             fixed: {},
//             source: {},
//             module: {},
//             arbitrary_content: {:#?},
//             indicativeness: {},
//             regex: Regex::new({}).unwrap(),
//             description: {},
//         }}"##,
//             self.path,
//             self.fixed,
//             self.source,
//             self.module,
//             self.arbitrary_content,
//             self.indicativeness,
//             self.regex.to_string(),
//             self.description,
//         )
//     }

//     pub fn from_csv_reader<R: std::io::Read>(rdr: &mut csv::Reader<R>) -> Result<Vec<Record>, ParseError> {
//         let mut records = vec!();
//         for result in rdr.deserialize() {
//             let record: Record = result?;
//             records.push(record);
//             // println!("{:?}", record);
//             // Try this if you don't like each record smushed on one line:
//             // println!("{:#?}", record);
//         }

//         Ok(records)
//     }

//     pub fn from_csv_read(csv_input: &mut dyn Read) -> Result<Vec<Record>, ParseError> {
//         let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(csv_input);
//         Self::from_csv_reader(&mut rdr)
//     }

//     // pub fn from_csv(csv_str: &str) -> Result<Vec<Record>, ParseError> {
//     //     // log::debug!("Parsing CSV to Records ...");
//     //     // let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());

//     //     // let mut records = vec!();
//     //     // for result in rdr.deserialize() {
//     //     //     let record: Record = result?;
//     //     //     records.push(record);
//     //     //     // println!("{:?}", record);
//     //     //     // Try this if you don't like each record smushed on one line:
//     //     //     // println!("{:#?}", record);
//     //     // }

//     //     let mut csv_bytes = csv_str.as_bytes_mut();
//     //     Self::from_csv_reader(csv_bytes)
//     // }

//     pub fn from_csv_file(csv_file: &Path) -> Result<Vec<Record>, ParseError> {
//         // log::debug!("Reading CSV file to string ...");
//         // let csv_str = fs::read_to_string(csv_file)?;

//         // Self::from_csv(&csv_str)

//         let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(fs::File::open(csv_file)?);
//         Self::from_csv_reader(&mut rdr)
//     }

//     // pub fn to_yaml(&self) -> Result<String, SerError> {
//     //     log::debug!("Serializing v1 to YAML ...");
//     //     Ok(serde_yaml::to_string(self)?)
//     // }

//     // pub fn to_yaml_file(&self, yaml_file: &Path) -> Result<(), SerError> {
//     //     let serialized = self.to_yaml()?;
//     //     log::debug!("Writing v1 to YAML file ...");
//     //     fs::write(yaml_file, serialized)?;
//     //     Ok(())
//     // }

//     // pub fn ext_matcher() -> &'static Regex {
//     //     rgx!(r#"(^|\.)[cC][sS][vV]$"#)
//     // }
// }
