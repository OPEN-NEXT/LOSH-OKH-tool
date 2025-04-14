// SPDX-FileCopyrightText: 2021 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod cli;
mod conversion;
mod dir;
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
    env::{self, current_dir},
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use clap::Command;
use formats::{v1, v2};
use log::LevelFilter;

macro_rules! main_err {
    ($msg:expr) => {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, $msg).into())
    };
}

/**
 * Returns the path to the parent dir of the argument,
 * if it has one.
 * We use this rather complex way of doing it,
 * because with a simple `file_path.parent()?.exists()`,
 * we would get `false` in case of `"bla.txt"`,
 * even if CWD is an existing directory.
 */
fn get_parent<P>(file_path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let file_path_val = file_path.as_ref();
    let parent = file_path_val.parent()?;
    if parent.components().next().is_none() {
        return current_dir().ok();
    }
    Some(parent.to_owned())
}

#[allow(clippy::fn_params_excessive_bools)]
fn convert<IP, OP>(
    input_path: IP,
    output_path: Option<OP>,
    recursive: bool,
    cont: bool,
    overwrite: bool,
    quiet: bool,
) -> Result<(), Box<dyn Error>>
where
    IP: AsRef<Path>,
    OP: AsRef<Path>,
{
    if input_path.as_ref().is_file() {
        let output_path_val = if let Some(output_path_val) = output_path {
            if output_path_val.as_ref().exists() {
                if output_path_val.as_ref().is_file() {
                    output_path_val.as_ref().to_path_buf()
                } else {
                    main_err!("input is a file, so output would have to be too, but is not");
                }
            } else {
                let out_parent = get_parent(output_path_val.as_ref());
                if let Some(out_parent_val) = out_parent {
                    if !out_parent_val.exists() {
                        main_err!(format!(
                            "the output file's parent directory '{}' does not exist",
                            out_parent_val.display()
                        ));
                    }
                } else {
                    main_err!("failed to determine output file's parent directory");
                }
                output_path_val.as_ref().to_path_buf()
            }
        } else {
            let input_ext = input_path.as_ref().extension().and_then(OsStr::to_str);
            if let Some(input_ext_val) = input_ext {
                let input_ext_lower = input_ext_val.to_lowercase();
                if ["yml", "yaml"].contains(&input_ext_lower.as_str()) {
                    let mut output_path = input_path.as_ref().to_path_buf();
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
        let toml_file = output_path_val;
        if toml_file.exists() && !overwrite {
            log::info!(
                "Skipping conversion of '{}' to '{}', because the target file already exists (see --{})",
                yaml_file.as_ref().display(),
                toml_file.display(),
                cli::A_L_OVERWRITE
            );
        } else {
            conversion::v1_to_v2::convert_file(yaml_file, &toml_file)?;
        }
        Ok(())
    } else if input_path.as_ref().is_dir() {
        let output_path_val = match output_path {
            Some(output_path_bare) => {
                if output_path_bare.as_ref().is_dir() {
                    output_path_bare.as_ref().to_path_buf()
                } else {
                    main_err!("input is a dir, so output would have to be too, but is not");
                }
            }
            None => input_path.as_ref().to_path_buf(),
        };

        let mut total_res = Ok(());
        let file_matcher = v1::Okh::file_matcher();
        for yaml_file in dir::iter_exts(dir::walker(&input_path, recursive), v1::Okh::ext_matcher())
        {
            if let Some(yaml_file_name) = yaml_file.file_name() {
                if !file_matcher.is_match(&yaml_file_name.to_string_lossy()) {
                    continue;
                }
                let mut toml_file = output_path_val.join(yaml_file.strip_prefix(&input_path)?);
                toml_file.set_extension("toml");
                fs::create_dir_all(toml_file.parent().unwrap())?;
                if toml_file.exists() && !overwrite {
                    log::info!(
                        "Skipping conversion of '{}' to '{}', because the target file already exists (see --{})",
                        yaml_file.display(),
                        toml_file.display(),
                        cli::A_L_OVERWRITE
                    );
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
        }
        Ok(total_res?)
    } else {
        main_err!("input is neither a file nor a dir; do not know what to do");
    }
}

fn validate<IP>(
    input_path: IP,
    recursive: bool,
    okhv1: Option<bool>,
    quiet: bool,
) -> Result<(), Box<dyn Error>>
where
    IP: AsRef<Path>,
{
    if input_path.as_ref().is_file() {
        let okhv1_val = match okhv1 {
            Some(okhv1_val) => okhv1_val,
            None => {
                if v1::Okh::ext_matcher()
                    .is_match(input_path.as_ref().extension().unwrap().to_str().unwrap())
                {
                    // TODO get rid of the unwraps
                    true
                } else if v2::Okh::ext_matcher()
                    .is_match(input_path.as_ref().extension().unwrap().to_str().unwrap())
                {
                    // TODO get rid of the unwraps
                    false
                } else {
                    main_err!("unable to figure out OKH version from input file extension");
                }
            }
        };
        let validator = if okhv1_val {
            validation::okh_v1_yaml
        } else {
            validation::okh_losh_toml
        };
        Ok(validator(input_path)?)
    } else if input_path.as_ref().is_dir() {
        let okhv1_val = okhv1.unwrap_or_else(|| {
            panic!(
                "Input dir specified, but missing an OKH version to scan for, use --{} <OKH-VERSION>",
                cli::A_L_OKH_VERSION
            )
        });
        let ext_matcher = if okhv1_val {
            v1::Okh::ext_matcher()
        } else {
            v2::Okh::ext_matcher()
        };
        let file_matcher = if okhv1_val {
            v1::Okh::file_matcher()
        } else {
            v2::Okh::file_matcher()
        };
        let validator = if okhv1_val {
            validation::okh_v1_yaml
        } else {
            validation::okh_losh_toml
        };
        let mut errors = Vec::new();
        let mut manifests_processed = 0;
        for input_file in dir::iter_exts(dir::walker(&input_path, recursive), ext_matcher) {
            if let Some(input_file_name) = input_file.file_name() {
                if !file_matcher.is_match(&input_file_name.to_string_lossy()) {
                    continue;
                }
                let single_res = validator(input_file.clone());
                if let Err(err) = single_res {
                    errors.push((input_file, err));
                }
                manifests_processed += 1;
            }
        }
        let total_res: Result<(), validation::ErrorCollection> = if !errors.is_empty() {
            if errors.len() == 1 {
                Err(errors.into_iter().next().unwrap().into())
            } else {
                Err(validation::ErrorCollection { errors })
            }
        } else if manifests_processed > 0 {
            Ok(())
        } else {
            Err(validation::ErrorCollection::from((
                input_path.as_ref().to_path_buf(),
                validation::Error::NoManifestsFound,
            )))
        };
        Ok(total_res?)
    } else {
        main_err!("input is neither a file nor a dir; do not know what to do");
    }
}

fn generate(overwrite: bool, quiet: bool) -> Result<(), Box<dyn Error>> {
    let proj_root = env::current_dir()?;
    Ok(generation::okh_losh_toml(&proj_root, overwrite)?)
}

fn print_version_and_exit(quiet: bool) {
    #![allow(clippy::print_stdout)]

    if !quiet {
        print!("{} ", clap::crate_name!());
    }
    println!("{}", okh_tool::VERSION);
    std::process::exit(0);
}

fn main_inner() -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        logger::init(None, (LevelFilter::Trace, LevelFilter::Trace));
    } else {
        logger::init(None, (LevelFilter::Info, LevelFilter::Trace));
    }

    let arg_matcher = cli::arg_matcher();
    let sub_command_names: Vec<String> = arg_matcher
        .get_subcommands()
        .map(Command::get_name)
        .map(ToOwned::to_owned)
        .collect();
    let args = &arg_matcher.get_matches();
    let quiet = args.get_flag(cli::A_L_QUIET);
    let version = args.get_flag(cli::A_L_VERSION);
    if version {
        print_version_and_exit(quiet);
    }
    if sub_command_names.is_empty() {
        log::error!(
            "'{}' requires a subcommand but none was provided",
            clap::crate_name!()
        );
        cli::arg_matcher().print_help()?;
        std::process::exit(1);
    }
    for sub_com_name in &sub_command_names {
        if let Some(sub_com) = args.subcommand_matches(sub_com_name) {
            if sub_com_name == cli::SC_N_CONVERT {
                let input_path = sub_com.get_one::<PathBuf>(cli::A_P_INPUT).unwrap();
                let output_path = sub_com.get_one::<PathBuf>(cli::A_P_OUTPUT);
                let recursive = sub_com.get_flag(cli::A_L_RECURSIVE);
                let cont = sub_com.get_flag(cli::A_L_CONTINUE_ON_ERROR);
                let overwrite = sub_com.get_flag(cli::A_L_OVERWRITE);
                convert(input_path, output_path, recursive, cont, overwrite, quiet)?;
            } else if sub_com_name == cli::SC_N_VALIDATE {
                let input_path = sub_com.get_one::<PathBuf>(cli::A_P_INPUT).unwrap();
                let recursive = sub_com.get_flag(cli::A_L_RECURSIVE);
                let okhv1 = sub_com
                    .get_one::<String>(cli::A_L_OKH_VERSION)
                    .map(|ver| ver == "v1");
                validate(input_path, recursive, okhv1, quiet)?;
                log::info!("Valid!");
            } else if sub_com_name == cli::SC_N_GENERATE {
                let overwrite = sub_com.get_flag(cli::A_L_OVERWRITE);
                generate(overwrite, quiet)?;
            } else {
                main_err!(format!("Sub-command not implemented: '{sub_com_name}'"));
            }
        }
    }
    Ok(())
}

fn main() {
    main_inner().unwrap_or_else(|err| {
        log::error!("{err:#?}");
        std::process::exit(1);
    });
}
