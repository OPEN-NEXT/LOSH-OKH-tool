// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// This contians types for the data stored in
/// `resources/osh-file-types/file_extension_formats-*.csv`,
/// which is loaded by `build.rs` and written to `file_types.rs`.
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Openness {
    Open,
    Proprietary,
    Unknown,
}

impl From<Option<bool>> for Openness {
    fn from(val: Option<bool>) -> Self {
        match val {
            Some(true) => Self::Open,
            Some(false) => Self::Proprietary,
            None => Self::Unknown,
        }
    }
}

impl TryFrom<&str> for Openness {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "open" => Ok(Self::Open),
            "proprietary" => Ok(Self::Proprietary),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Invalid Open column value: '{val}'")),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Text,
    Binary,
    Both,
    Unknown,
}

// impl From<Option<bool>> for Text {
//     fn from(val: Option<bool>) -> Self {
//         match val {
//             Some(true) => Self::Text,
//             Some(false) => Self::Binary,
//             Some(false) => Self::Both,
//             None => Self::Unknown,
//         }
//     }
// }

impl TryFrom<&str> for Format {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "text" => Ok(Self::Text),
            "binary" => Ok(Self::Binary),
            "both" => Ok(Self::Both),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Invalid Text column value: '{val}'")),
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Source,
    Export,
}

impl From<bool> for Source {
    fn from(val: bool) -> Self {
        if val { Self::Source } else { Self::Export }
    }
}

impl TryFrom<&str> for Source {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "source" => Ok(Self::Source),
            "export" => Ok(Self::Export),
            _ => Err(format!("Invalid Source column value: '{val}'")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileFormat<S> {
    pub extension: S,
    pub open: Openness,
    pub text: Format,
    pub source: Source,
}
