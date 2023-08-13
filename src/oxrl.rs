// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, IntoStaticStr};

// lazy_static! {
//     static ref R_NON_SIMPLE: Regex = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
//     static ref R_TO_UNDERSCORE: Regex = Regex::new(r"[ -]").unwrap();
//     static ref R_OTRL_PREFIX: Regex = Regex::new(r"^otrl").unwrap();
//     static ref R_ODRL_PREFIX: Regex = Regex::new(r"^odrl").unwrap();
// }

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Tried to convert a number ({0}) to the enum, which was not in its range.")]
    InvalidIndex(u8),
}

macro_rules! impl_oxrl {
    ($name:ident, $fmt_prefix:expr, $r_prefix:ident) => {
        impl From<$name> for u8 {
            fn from(oxrl: $name) -> Self {
                oxrl as u8
            }
        }

        impl TryFrom<u8> for $name {
            type Error = ParseError;

            fn try_from(num: u8) -> Result<Self, Self::Error> {
                let parsed: Option<$name> = FromPrimitive::from_u8(num);
                parsed.ok_or_else(|| ParseError::InvalidIndex(num))
            }
        }

        // impl Display for $name {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         f.write_fmt(format_args!(concat!($fmt_prefix, "-{}"), *self as u8))
        //     }
        // }

        // impl FromStr for $name {
        //     type Err = ParseError;

        //     fn from_str(s: &str) -> Result<Self, Self::Err> {
        //         // TODO This following mechanism is not optimal, but ok for now.
        //         let lower = s.to_lowercase();
        //         let clean = R_NON_SIMPLE.replace(&lower, "");
        //         let cleaner = R_TO_UNDERSCORE.replace(&clean, "_");
        //         let no_prefix = $r_prefix.replace(&cleaner, "");
        //         let num = u8::from_str(&no_prefix);
        //         if let Ok(num) = num {
        //             $name::try_from(num)
        //         } else {
        //             // match
        //             $name::from_str(&no_prefix) // TODO FIX endless loop here! :D
        //         }
        //     }
        // }
    };
}

#[derive(
    IntoStaticStr,
    Display,
    EnumString,
    FromPrimitive,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum Otrl {
    /// Product idea; needs are identified and initial specifications are defined.
    #[serde(rename = "OTRL-1")]
    Ideation = 1,
    /// Mature product concept has been formulated
    #[serde(rename = "OTRL-2")]
    Conception = 2,
    /// Product model is developed
    #[serde(rename = "OTRL-3")]
    Development = 3,
    /// Full functional prototype is built and tested
    #[serde(rename = "OTRL-4")]
    PrototypingAndTesting = 4,
    /// Fairly reliable processes identified and characterised
    #[serde(rename = "OTRL-5")]
    ManufacturingDevelopment = 5,
    /// Certificate marking conformity assessment or comparable
    #[serde(rename = "OTRL-6")]
    ProductQualification = 6,
}

impl_oxrl!(Otrl, "OTRL", R_OTRL_PREFIX);

#[derive(
    IntoStaticStr,
    Display,
    EnumString,
    FromPrimitive,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum Odrl {
    /// Published information under free open source licence
    #[serde(rename = "ODRL-1")]
    Started = 1,
    /// Provision of documentation files and in editable formats enabling collaboration development
    #[serde(rename = "ODRL-2")]
    Minimal = 2,
    /// Complete documentation as per DIN SPEC 3105-1
    #[serde(rename = "ODRL-3")]
    Full = 3,
    /// Public evidence of documentation maturity
    #[serde(rename = "ODRL-3*")]
    Audited = 4,
    /// Full documentation for product qualification
    #[serde(rename = "ODRL-4")]
    QualificationReady = 5,
}

impl_oxrl!(Odrl, "ODRL", R_ODRL_PREFIX);
