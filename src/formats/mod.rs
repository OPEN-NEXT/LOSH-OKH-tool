// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
