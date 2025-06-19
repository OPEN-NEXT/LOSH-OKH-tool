// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process,
};
use typify::{TypeSpace, TypeSpaceSettings};
use schemars::schema::RootSchema;

#[path = "src/file_types_format.rs"]
mod file_types_format;

use file_types_format::FileFormat;

const OSH_FILE_TYPES_ROOT: &str = "resources/osh-file-types";

fn transcribe_file_ext(dest_file: &mut File, category: &str) -> Result<(), Box<dyn Error>> {
    let in_file = fs::canonicalize(PathBuf::from(format!(
        "{OSH_FILE_TYPES_ROOT}/file_extension_formats-{category}.csv"
    )))?;
    println!("cargo:rerun-if-changed={}", in_file.display());
    let mut rdr = csv::Reader::from_path(in_file)?;

    let mut formats = vec![];
    for record_res in rdr.records() {
        let record = record_res?;
        // NOTE We need to provide a type hint for automatic deserialization.
        let format: FileFormat<String> = FileFormat {
            extension: record[0].into(),
            open: record[1].try_into()?,
            text: record[2].try_into()?,
            source: record[3].try_into()?,
        };
        formats.push(format);
    }

    writeln!(
        dest_file,
        "\npub const {}: [FileFormat<&'static str>; {}] = [",
        category.to_uppercase(),
        formats.len()
    )?;
    for format in &formats {
        writeln!(dest_file, "    FileFormat {{")?;
        writeln!(dest_file, "        extension: \"{}\",", format.extension)?;
        writeln!(dest_file, "        open: Openness::{:?},", format.open)?;
        writeln!(dest_file, "        text: Format::{:?},", format.text)?;
        writeln!(dest_file, "        source: Source::{:?},", format.source)?;
        writeln!(dest_file, "    }},")?;
    }
    writeln!(dest_file, "];\n")?;

    write!(
        dest_file,
        r#"pub const RS_{}: &str = r"^("#,
        category.to_uppercase()
    )?;
    let mut fmt_iter = formats.into_iter();
    write!(
        dest_file,
        "{}",
        fmt_iter.next().ok_or("No first format")?.extension
    )?;
    let res: Result<(), std::io::Error> =
        fmt_iter.try_for_each(|fmt| write!(dest_file, "|{}", fmt.extension));
    res?;
    writeln!(dest_file, r#")$";"#)?;

    Ok(())
}

fn transcribe_file_exts() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("file_types.rs");
    let categories = vec!["cad", "pcb"];

    let mut dest_file = File::create(dest_path)?;

    writeln!(
        dest_file,
        "use crate::file_types_format::{{FileFormat, Openness, Format, Source}};"
    )?;

    for category in categories {
        transcribe_file_ext(&mut dest_file, category)?;
    }

    Ok(())
}

fn convert_okh_schema_to_types() {
    let schema_file = "resources/okh/src/schema/okh.schema.json";
    println!("cargo:rerun-if-changed={schema_file}");
    let content = std::fs::read_to_string(schema_file).unwrap();
    let schema = serde_json::from_str::<RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("okh_model_v_2_4.rs");
    fs::write(out_file, contents).unwrap();
}

fn main() {
    if let Err(err) = transcribe_file_exts() {
        println!("error running transcribe_file_exts(): {err}");
        process::exit(1);
    }

    // convert_okh_schema_to_types();
}
