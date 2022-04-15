// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

mod cli;
mod codify;
mod conversion;
mod dir;
mod dir_std;
mod file_types;
mod file_types_format;
mod formats;
mod generation;
mod license;
mod logger;
mod macros;
mod oxrl;
mod validation;

use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use clap::App;
use formats::{v1, v2};
use log::LevelFilter;

macro_rules! main_err {
    ($msg:expr) => {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, $msg).into())
    };
}

fn convert(
    input_path: PathBuf,
    output_path: Option<&Path>,
    recursive: bool,
    cont: bool,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        let output_path = if let Some(output_path) = output_path {
            if output_path.exists() {
                if output_path.is_file() {
                    output_path.to_path_buf()
                } else {
                    main_err!("input is a file, so output would have to be too, but is not");
                }
            } else {
                // let out_parent_abs = output_path.canonicalize()?;
                let out_parent = output_path.parent();
                if let Some(out_parent) = out_parent {
                    if !out_parent.exists() {
                        main_err!("the output file's parent directory does not exist");
                    }
                } else {
                    main_err!("failed to determine output file's parent directory");
                }
                output_path.to_path_buf()
            }
        } else {
            let input_ext = input_path.extension().and_then(OsStr::to_str);
            if let Some(input_ext) = input_ext {
                let input_ext_lower = input_ext.to_lowercase();
                if ["yml", "yaml"].contains(&input_ext_lower.as_str()) {
                    let mut output_path = input_path.clone();
                    output_path.set_extension("toml");
                    output_path
                } else {
                    main_err!("input file has an unsupported file extension");
                }
            } else {
                main_err!("input file has a non-unicode file extension");
            }
        };

        let yaml_file = input_path;
        let toml_file = output_path;
        if toml_file.exists() && !overwrite {
            log::info!("Skipping conversion of '{}' to '{}', because the target file already exists (see --{})", yaml_file.display(), toml_file.display(), cli::A_L_OVERWRITE);
        } else {
            conversion::v1_to_v2::convert_file(&yaml_file, &toml_file)?;
        }
        Ok(())
    } else if input_path.is_dir() {
        let output_path = match output_path {
            Some(output_path) => {
                if output_path.is_dir() {
                    output_path
                } else {
                    main_err!("input is a dir, so output would have to be too, but is not");
                }
            }
            None => &input_path,
        };

        let mut total_res = Ok(());
        for yaml_file in dir::iter_exts(dir::walker(&input_path, recursive), v1::Okh::ext_matcher())
        {
            let mut toml_file = output_path.join(yaml_file.strip_prefix(&input_path)?);
            toml_file.set_extension("toml");
            fs::create_dir_all(toml_file.parent().unwrap())?;
            if toml_file.exists() && !overwrite {
                log::info!("Skipping conversion of '{}' to '{}', because the target file already exists (see --{})", yaml_file.display(), toml_file.display(), cli::A_L_OVERWRITE);
                continue;
            }
            let res = conversion::v1_to_v2::convert_file(&yaml_file, &toml_file);
            if let Err(err) = res {
                log::warn!("File: '{}'\n{}", yaml_file.display(), &err);
                total_res = Err(err); // TODO FIXME We need a simple "Not all succeeded" indicator error here!
                if !cont {
                    break;
                }
            }
        }
        Ok(total_res?)
    } else {
        main_err!("input is neither a file nor a dir; do not know what to do");
    }
}

fn validate(
    input_path: &Path,
    recursive: bool,
    okhv1: Option<bool>,
    cont: bool,
) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        let okhv1 = match okhv1 {
            Some(okhv1) => okhv1,
            None => {
                if v1::Okh::ext_matcher()
                    .is_match(input_path.extension().unwrap().to_str().unwrap())
                {
                    // TODO get rid of the unwraps
                    true
                } else if v2::Okh::ext_matcher()
                    .is_match(input_path.extension().unwrap().to_str().unwrap())
                {
                    // TODO get rid of the unwraps
                    false
                } else {
                    main_err!("unable to figure out OKH version from input file extension");
                }
            }
        };
        if okhv1 {
            validation::okh_v1_yaml(input_path)?;
        } else {
            validation::okh_losh_toml(input_path)?;
        }
        Ok(())
    } else if input_path.is_dir() {
        let okhv1 = okhv1.unwrap_or_else(|| {
            panic!(
                "Input dir specified, but missing an OKH version to scan for, see --{}",
                cli::A_L_OKH_VERSION
            )
        });
        let ext_matcher = if okhv1 {
            v1::Okh::ext_matcher()
        } else {
            v2::Okh::ext_matcher()
        };
        let validator = if okhv1 {
            validation::okh_v1_yaml
        } else {
            validation::okh_losh_toml
        };
        let mut total_res = Ok(());
        for input_file in dir::iter_exts(dir::walker(input_path, recursive), ext_matcher) {
            let res = validator(&input_file);
            if let Err(err) = res {
                log::warn!("File: '{}'\n{}", input_file.display(), &err);
                total_res = Err(err); // TODO FIXME We need a simple "Not all succeeded" indicator error here!
                if !cont {
                    break;
                }
            }
        }
        Ok(total_res?)
    } else {
        main_err!("input is neither a file nor a dir; do not know what to do");
    }
}

fn generate(overwrite: bool) -> Result<(), Box<dyn Error>> {
    let proj_root = env::current_dir()?;
    Ok(generation::okh_losh_toml(&proj_root, overwrite)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    logger::init(None, (LevelFilter::Trace, LevelFilter::Trace));
    // logger::init(None, (LevelFilter::Info, LevelFilter::Trace));

    let arg_matcher = cli::arg_matcher();
    let sub_command_names: Vec<String> = arg_matcher
        .get_subcommands()
        .map(App::get_name)
        .map(ToOwned::to_owned)
        .collect();
    let args = &arg_matcher.get_matches();
    for sub_com_name in &sub_command_names {
        if let Some(sub_com) = args.subcommand_matches(sub_com_name) {
            if sub_com_name == cli::SC_N_CONVERT {
                let input_path = sub_com
                    .value_of(cli::A_P_INPUT)
                    .map(Path::new)
                    .map(Path::to_path_buf)
                    .unwrap();
                let output_path = sub_com.value_of(cli::A_P_OUTPUT).map(Path::new);
                let recursive = sub_com.is_present(cli::A_L_RECURSIVE);
                let cont = sub_com.is_present(cli::A_L_CONTINUE_ON_ERROR);
                let overwrite = sub_com.is_present(cli::A_L_OVERWRITE);
                convert(input_path, output_path, recursive, cont, overwrite)?;
            } else if sub_com_name == cli::SC_N_VALIDATE {
                let input_path = sub_com
                    .value_of(cli::A_P_INPUT)
                    .map(Path::new) /*.map(Path::to_path_buf)*/
                    .unwrap();
                let recursive = sub_com.is_present(cli::A_L_RECURSIVE);
                let okhv1 = sub_com
                    .value_of(cli::A_L_OKH_VERSION)
                    .map(|ver| ver == "v1");
                let cont = sub_com.is_present(cli::A_L_CONTINUE_ON_ERROR);
                validate(input_path, recursive, okhv1, cont)?;
            } else if sub_com_name == cli::SC_N_GENERATE {
                let overwrite = sub_com.is_present(cli::A_L_OVERWRITE);
                generate(overwrite)?;
            } else {
                main_err!(format!("Sub-command not implemented: '{}'", sub_com_name));
            }
        }
    }
    Ok(())
}
