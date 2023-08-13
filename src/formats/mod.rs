// SPDX-FileCopyrightText: 2021-2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use relative_path::RelativePathBuf;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use url::Url;

pub mod v1;
pub mod v2;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Failed to read data to be parsed (e.g. from a file)")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse TOML")]
    Toml(#[from] toml::de::Error),

    #[error("Failed to parse YAML")]
    Yaml(#[from] serde_yaml::Error),
}

/// Serialization Error
#[derive(thiserror::Error, Debug)]
pub enum SerError {
    #[error("Failed to write data to be parsed (e.g. to a file)")]
    IO(#[from] std::io::Error),

    #[error("Failed to serialize TOML")]
    Toml(#[from] toml::ser::Error),

    #[error("Failed to serialize YAML")]
    Yaml(#[from] serde_yaml::Error),
}

/// A (serde compatible) property type,
/// representing either a URL or a (repo relative) path.
#[derive(Debug, PartialEq, Eq)]
pub enum Locator {
    Url(Url),
    Path(RelativePathBuf),
}

impl Locator {
    pub fn to_url(&self, repo_url: &Url) -> Url {
        match self {
            Self::Url(url) => url.clone(),
            Self::Path(rel_path) => {
                let repo_path = RelativePathBuf::from(repo_url.path());
                let mut specific_url = repo_url.clone();
                specific_url.set_path(repo_path.join(rel_path).as_str());
                specific_url
            }
        }
    }
}

impl<'de> Deserialize<'de> for Locator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocatorVisitor;

        impl<'de> Visitor<'de> for LocatorVisitor {
            type Value = Locator;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("`Url` or `RelativePathBuf`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Locator, E>
            where
                E: de::Error,
            {
                Ok(Url::parse(value).map_or_else(
                    |_| Locator::Path(RelativePathBuf::from(value)),
                    Locator::Url,
                ))
            }
        }

        deserializer.deserialize_str(LocatorVisitor)
    }
}

impl Serialize for Locator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self {
            Self::Url(url) => url.as_str(),
            Self::Path(relative_path) => relative_path.as_str(),
        };
        serializer.serialize_str(value)
    }
}
