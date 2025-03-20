// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// use jsonschema::{Draft, JSONSchema};
use jsonschema::{Draft, Validator};
// use serde_json::json;
use once_cell::sync::Lazy;

use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use crate::license;

const SCHEMA_OKH_LOSH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/okh/src/schema/okh.schema.json"
));
const SCHEMA_OKH_V1: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/okh/src/schema/okh-v1.schema.json"
));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to find any manifests.")]
    NoManifestsFound,

    #[error("Failed to read or write, probably from/to the file-system.")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse the raw content as TOML.")]
    TomlParseFailure(#[from] toml::de::Error),

    #[error("Failed to parse the raw content as JSON.")]
    JsonParseFailure(#[from] serde_json::error::Error),

    #[error("Failed to parse the raw content as YAML.")]
    YamlParseFailure(#[from] serde_yaml::Error),

    #[error("Failed to validate schema content {0:#}.")]
    InvalidContent(#[from] JsonSchemaValidationError),

    // #[error(transparent)]
    #[error("Failed to validate:\n{0:#}")]
    ValidationFailure(#[from] JsonSchemaValidationErrorCollection),

    #[error("License issue:\n{0:#}")]
    License(#[from] license::Error),
}

#[derive(thiserror::Error, Debug)]
pub struct ErrorCollection {
    /// Failed requirements.
    pub errors: Vec<(PathBuf, Error)>,
}

impl fmt::Display for ErrorCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n# Validation error(s):\n")?;
        for (file, err) in &self.errors {
            // f.write_fmt(format_args!("Validation error: {}\nInstance path: {}", failure, failure.instance_path))?;
            f.write_fmt(format_args!("In file '{}':\n{err}", file.display()))?;
        }
        Ok(())
    }
}

impl From<(PathBuf, Error)> for ErrorCollection {
    fn from(file_and_err: (PathBuf, Error)) -> Self {
        Self {
            errors: vec![file_and_err],
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Error:\n\tKind:    {kind:?}\n\tWhere:   {instance_path}\n\tContent: {instance}\n")]
pub struct JsonSchemaValidationError {
    /// Value of the property that failed validation.
    pub instance: serde_json::Value,
    /// Type of validation error.
    pub kind: jsonschema::error::ValidationErrorKind,
    /// Path to the value that failed validation.
    pub instance_path: jsonschema::paths::Location,
    /// Path to the JSON Schema keyword that failed validation.
    pub schema_path: jsonschema::paths::Location,
}

impl<'a> From<jsonschema::ValidationError<'a>> for JsonSchemaValidationError {
    fn from(err: jsonschema::ValidationError<'a>) -> Self {
        Self {
            instance: err.instance.into_owned(),
            kind: err.kind,
            instance_path: err.instance_path,
            schema_path: err.schema_path,
        }
    }
}

#[derive(thiserror::Error, Debug)]
// #[error("The source meta-data specifies no license.")]
pub struct JsonSchemaValidationErrorCollection {
    /// Failed requirements.
    pub failed_reqs: Vec<JsonSchemaValidationError>,
}

impl fmt::Display for JsonSchemaValidationErrorCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n# Validation error(s):\n")?;
        for failure in &self.failed_reqs {
            f.write_fmt(format_args!("{failure}"))?;
        }
        Ok(())
    }
}

impl<'a> From<jsonschema::ErrorIterator<'a>> for JsonSchemaValidationErrorCollection {
    fn from(err: jsonschema::ErrorIterator<'a>) -> Self {
        Self {
            failed_reqs: err.map(JsonSchemaValidationError::from).collect(),
        }
    }
}

/// Use this if you evaluate multiple contents (usually files)
/// with the same schema.
pub fn with_schema(schema: &Validator, content: &serde_json::Value) -> Result<(), Error> {
    schema
        .validate(content)
        .map_err(JsonSchemaValidationError::from)
        .map_err(std::convert::Into::into)
    // let result = schema.validate(content);
    // if let Err(errors) = result {
    //     for error in errors {
    //         log::error!("Validation error: {}", error);
    //         log::error!("Instance path: {}", error.instance_path);
    //     }
    // }

    // // result?;
    // Ok(())
}

pub fn okh_losh_toml<IP>(toml_path: IP) -> Result<(), Error>
where
    IP: AsRef<Path>,
{
    static RAW_SCHEMA: Lazy<serde_json::Value> = Lazy::new(|| {
        serde_json::from_str::<serde_json::Value>(SCHEMA_OKH_LOSH)
            .expect("The OKH-LOSH JSON schema contained within the binary is invalid JSON :/")
    });

    log::debug!(
        "Validating an OKH LOSH file ('{}') ...",
        toml_path.as_ref().as_os_str().to_str().unwrap()
    );
    let toml_str = fs::read_to_string(toml_path)?;
    let instance = toml::from_str::<serde_json::Value>(&toml_str)?;

    let validator = jsonschema::options()
        .with_draft(Draft::Draft7)
        .build(&RAW_SCHEMA)
        .map_err(JsonSchemaValidationError::from)?;
    with_schema(&validator, &instance)?;

    if let Some(license_val) = instance.get("license") {
        if let Some(license_str) = license_val.as_str() {
            license::validate_spdx_expr(license_str, false)?;
        }
    }

    Ok(())
}

pub fn okh_v1_yaml<IP>(yaml_path: IP) -> Result<(), Error>
where
    IP: AsRef<Path>,
{
    static RAW_SCHEMA: Lazy<serde_json::Value> = Lazy::new(|| {
        serde_json::from_str::<serde_json::Value>(SCHEMA_OKH_V1)
            .expect("The OKH-V1 JSON schema contained within the binary is invalid JSON :/")
    });

    log::debug!(
        "Validating an OKH v1 file ('{}') ...",
        yaml_path.as_ref().as_os_str().to_str().unwrap()
    );
    let yaml_str = fs::read_to_string(yaml_path)?;
    let instance = serde_yaml::from_str::<serde_json::Value>(&yaml_str)?;

    let validator = jsonschema::options()
        .with_draft(Draft::Draft7)
        .build(&RAW_SCHEMA)
        .map_err(JsonSchemaValidationError::from)?;
    with_schema(&validator, &instance)
}

/* pub fn list_files_by_ext(dir: &Path, extension: &str) -> Result<Vec<PathBuf>, Error> { */
/*     Ok(fs::read_dir(dir)? */
/*         .filter_map(|entry| { */
/*             entry.ok().and_then(|e| { */
/*                 e.path() */
/*                     .file_name() */
/*                     .and_then(|n| n.to_str().map(String::from)) */
/*                     .and_then(|fname| { */
/*                         if fname.ends_with(extension) { */
/*                             Some(e.path()) */
/*                         } else { */
/*                             None */
/*                         } */
/*                     }) */
/*             }) */
/*         }) */
/*         .collect()) */
/* } */

/* pub fn sample() -> Result<(), Error> { */
/*     // TODO get rid of this, but use its parts elsewhere */
/*     log::info!("Running sample validations ..."); */
/*     // let schema = serde_json::from_str::<serde_json::Value>(SCHEMA_OKH_LOSH)?; */
/*     // let schema_boxed: &'static serde_json::Value = Box::leak(Box::new(schema)); // TODO Cool Box::leak hack here! not yet understood.. but kind of important to keep! */

/*     okh_v1_yaml(&PathBuf::from_str("/home/hoijui/Projects/OSEG/repos/LOSH-okh-v1-to-v2/target/okh_v1/www_appropedia_org____okh_php_title_3D_printed_acorn_sheller.yml").unwrap())?; // TODO HACK Absolute path! */
/*     okh_losh_toml( */
/*         &PathBuf::from_str("/home/hoijui/Projects/OSEG/repos/LOSH/sample_data/okh-OHLOOM.toml") */
/*             .unwrap(), */
/*     )?; // TODO HACK Absolute path! */
/*     okh_losh_toml( */
/*         &PathBuf::from_str("/home/hoijui/Projects/OSEG/repos/LOSH/sample_data/okh-TEMPLATE.toml") */
/*             .unwrap(), */
/*     )?; // TODO HACK Absolute path! */

/*     let okh_v1_ymls = list_files_by_ext( */
/*         &PathBuf::from("/home/hoijui/Projects/OSEG/repos/LOSH-okh-v1-to-v2/target/okh_v1/"), */
/*         ".yml", */
/*     )?; // TODO HACK Absolute path! */
/*     log::info!("Validating *many* OKH v1 files ..."); */
/*     for okh_v1_yml in okh_v1_ymls { */
/*         // okh_v1_yaml(&okh_v1_yml)?; */
/*         if let Err(err) = okh_v1_yaml(&okh_v1_yml) { */
/*             log::error!( */
/*                 "Failed validating '{}'.", */
/*                 okh_v1_yml.as_os_str().to_str().unwrap() */
/*             ); */
/*             log::error!("{}", err); */
/*         } */
/*     } */

/*     Ok(()) */
/* } */
