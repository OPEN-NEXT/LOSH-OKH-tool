// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod v1_to_v2;
// pub mod v1_to_v2_4_generated;
// pub mod v1_to_v2_4;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The source meta-data specifies no license.")]
    NoLicense,

    #[error("Insufficient data found in source - {msg}")]
    InsufficientData { msg: &'static str },

    #[error("Some problem with git, for example, no repo present, or no branch/tag checked out")]
    GitProblem(#[from] git2::Error),

    #[error("Failed to parse the input format")]
    Parse(#[from] crate::formats::ParseError),

    #[error("Failed to serialize into the output format")]
    Serialize(#[from] crate::formats::SerError),
}
