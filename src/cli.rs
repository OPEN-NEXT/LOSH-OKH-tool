// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::{command, value_parser, Arg, ArgAction, Command, ValueHint};
use const_format::formatcp;
use std::{env, path::PathBuf};

pub const SC_N_VALIDATE: &str = "val";

pub const A_P_INPUT: &str = "INPUT";
pub const A_P_OUTPUT: &str = "OUTPUT";

pub const A_L_VERSION: &str = "version";
pub const A_S_VERSION: char = 'V';

pub const A_L_QUIET: &str = "quiet";
pub const A_S_QUIET: char = 'q';

pub const A_L_RECURSIVE: &str = "recursive";
pub const A_S_RECURSIVE: char = 'r';

pub const SC_N_CONVERT: &str = "conv";

pub const A_L_OKH_VERSION: &str = "okh-version";
pub const A_S_OKH_VERSION: char = 'o';

pub const A_L_CONTINUE_ON_ERROR: &str = "continue";
pub const A_S_CONTINUE_ON_ERROR: char = 'c';

pub const A_L_OVERWRITE: &str = "overwrite";
pub const A_S_OVERWRITE: char = 'o';

pub const SC_N_GENERATE: &str = "gen";

pub const OKH_MANIFEST_FILE_NAME: &str = "okh.toml";

fn arg_input() -> Arg {
    Arg::new(A_P_INPUT)
        .help("The input file or dir path")
        .num_args(1)
        .value_name("INPUT")
        .value_hint(ValueHint::AnyPath)
        .value_parser(value_parser!(PathBuf))
        .action(ArgAction::Set)
        .required(true)
}

fn arg_output() -> Arg {
    Arg::new(A_P_OUTPUT)
        .help("The output file or dir path")
        .num_args(1)
        .value_name("OUTPUT")
        .value_hint(ValueHint::AnyPath)
        .value_parser(value_parser!(PathBuf))
        .action(ArgAction::Set)
}

fn arg_recursive() -> Arg {
    Arg::new(A_L_RECURSIVE)
        .help("If the input path is a directory, search for input files recursively")
        .short(A_S_RECURSIVE)
        .long(A_L_RECURSIVE)
        .action(ArgAction::SetTrue)
}

fn arg_version() -> Arg {
    Arg::new(A_L_VERSION)
        .help("Print version information. May be combined with --quiet, to really only output the version string.")
        .short(A_S_VERSION)
        .long(A_L_VERSION)
        .action(ArgAction::SetTrue)
}

fn arg_quiet() -> Arg {
    Arg::new(A_L_QUIET)
        .help("None or much less command line output")
        .short(A_S_QUIET)
        .long(A_L_QUIET)
        .action(ArgAction::SetTrue)
}

fn subcom_convert() -> Command {
    Command::new(SC_N_CONVERT)
        .about("Converts one format into an other (currently only OKH-v1 to OKH-LOSH)")
        .arg(arg_input().index(1))
        .arg(arg_output().index(2))
        .arg(arg_recursive())
        .arg(arg_continue_on_error())
        .arg(arg_overwrite())
}

fn arg_okhv() -> Arg {
    Arg::new(A_L_OKH_VERSION)
        .help("If the input path is a directory, search for input files recursively")
        .num_args(1)
        .short(A_S_OKH_VERSION)
        .long(A_L_OKH_VERSION)
        .value_parser(["v1", "losh"])
        .action(ArgAction::Set)
}

fn arg_continue_on_error() -> Arg {
    Arg::new(A_L_CONTINUE_ON_ERROR)
        .help("If the input path is a directory, continue processing further files, even after an error")
        .short(A_S_CONTINUE_ON_ERROR)
        .long(A_L_CONTINUE_ON_ERROR)
        .action(ArgAction::SetTrue)
}

fn arg_overwrite() -> Arg {
    Arg::new(A_L_OVERWRITE)
        .help("If the output file already exists, overwrite it, instead of skipping the conversion")
        .short(A_S_OVERWRITE)
        .long(A_L_OVERWRITE)
        .action(ArgAction::SetTrue)
}

fn subcom_validate() -> Command {
    Command::new(SC_N_VALIDATE)
    .about("Validates manifest files for validity using JSON Schema (currently supports OKH-v1 and OKH-LOSH)")
    .arg(arg_input().index(1))
    .arg(arg_okhv())
    .arg(arg_recursive())
    .arg(arg_continue_on_error())
}

fn subcom_generate() -> Command {
    Command::new(SC_N_GENERATE)
    .about(formatcp!("Generates a starter-manifest file ('{}', OKH-LOSH) for the project at CWD. You will need to manually replace some values within it.", OKH_MANIFEST_FILE_NAME))
    .arg(arg_overwrite())
}

pub fn arg_matcher() -> Command {
    command!()
        .subcommand_negates_reqs(true)
        .propagate_version(true)
        .help_expected(true)
        .disable_version_flag(true)
        .bin_name(clap::crate_name!())
        .arg(arg_version())
        .arg(arg_quiet())
        .subcommand(subcom_convert())
        .subcommand(subcom_validate())
        .subcommand(subcom_generate())
}
