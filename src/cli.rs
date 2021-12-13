// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::{app_from_crate, App, AppSettings, Arg, ValueHint};
use std::env;

pub const SC_N_VALIDATE: &str = "val";

pub const A_P_INPUT: &str = "INPUT";
pub const A_P_OUTPUT: &str = "OUTPUT";

pub const A_L_RECURSIVE: &str = "recursive";
pub const A_S_RECURSIVE: char = 'r';

pub const SC_N_CONVERT: &str = "conv";

pub const A_L_OKH_VERSION: &str = "okh-version";
pub const A_S_OKH_VERSION: char = 'o';

pub const A_L_CONTINUE_ON_ERROR: &str = "continue";
pub const A_S_CONTINUE_ON_ERROR: char = 'c';

pub const A_L_OVERWRITE: &str = "overwrite";
pub const A_S_OVERWRITE: char = 'o';

fn arg_input() -> Arg<'static> {
    Arg::new(A_P_INPUT)
        .help("The input file or dir path")
        .takes_value(true)
        .value_name("INPUT")
        .value_hint(ValueHint::AnyPath) // TODO Add a validation function
        .required(true)
}

fn arg_output() -> Arg<'static> {
    Arg::new(A_P_OUTPUT)
        .help("The output file or dir path")
        .takes_value(true)
        .value_name("OUTPUT")
        .value_hint(ValueHint::AnyPath) // TODO Add a validation function
        .required(false)
}

fn arg_recursive() -> Arg<'static> {
    Arg::new(A_L_RECURSIVE)
        .help("If the input path is a directory, search for input files recursively")
        .takes_value(false)
        .short(A_S_RECURSIVE)
        .long(A_L_RECURSIVE)
        .required(false)
}

fn subcom_convert() -> App<'static> {
    App::new(SC_N_CONVERT)
        .about("Converts one format into an other (currently only OKH-v1 to OKH-LOSH)")
        .arg(arg_input().index(1))
        .arg(arg_output().index(2))
        .arg(arg_recursive())
        .arg(arg_continue_on_error())
        .arg(arg_overwrite())
}

fn arg_okhv() -> Arg<'static> {
    Arg::new(A_L_OKH_VERSION)
        .help("If the input path is a directory, search for input files recursively")
        .takes_value(true)
        .short(A_S_OKH_VERSION)
        .long(A_L_OKH_VERSION)
        .possible_values(["v1", "losh"])
        .required(false)
}

fn arg_continue_on_error() -> Arg<'static> {
    Arg::new(A_L_CONTINUE_ON_ERROR)
        .help("If the input path is a directory, continue processing further files, even after an error")
        .takes_value(false)
        .short(A_S_CONTINUE_ON_ERROR)
        .long(A_L_CONTINUE_ON_ERROR)
        .required(false)
}

fn arg_overwrite() -> Arg<'static> {
    Arg::new(A_L_OVERWRITE)
        .help("If the outout file alreayd exists, overwrite it, instead of skipping the conversion")
        .takes_value(false)
        .short(A_S_OVERWRITE)
        .long(A_L_OVERWRITE)
        .required(false)
}

fn subcom_validate() -> App<'static> {
    App::new(SC_N_VALIDATE)
    .about("Validates manifest files for validity using JSON Schema (currently supports OKH-v1 and OKH-LOSH)")
    .arg(arg_input().index(1))
    .arg(arg_okhv())
    .arg(arg_recursive())
    .arg(arg_continue_on_error())
}

pub fn arg_matcher() -> App<'static> {
    let app = app_from_crate!()
        .setting(AppSettings::HelpExpected)
        .setting(AppSettings::InferLongArgs)
        .setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::UseLongFormatForHelpSubcommand)
        .bin_name("okh-tool")
        .subcommand(subcom_convert())
        .subcommand(subcom_validate());
    app
}
