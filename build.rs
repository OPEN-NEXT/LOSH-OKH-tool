// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    process,
};

#[path = "src/codify.rs"]
mod codify;
#[path = "src/dir_std/format.rs"]
mod dir_std;
#[path = "src/file_types_format.rs"]
mod file_types_format;

use crate::codify::Codify;
use file_types_format::FileFormat;

const OSH_FILE_TYPES_ROOT: &str = "resources/osh-file-types";

fn transcribe_file_ext(dest_file: &mut File, category: &str) -> Result<(), Box<dyn Error>> {
    let in_file = format!(
        "{}/file_extension_formats-{}.csv",
        OSH_FILE_TYPES_ROOT, category
    );
    let in_file = fs::canonicalize(Path::new(&in_file))?;
    println!("cargo:rerun-if-changed={}", in_file.display());
    let mut rdr = csv::Reader::from_path(in_file)?;

    let mut formats = vec![];
    for record in rdr.records() {
        let record = record?;
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
        writeln!(dest_file, "        open: Open::{:?},", format.open)?;
        writeln!(dest_file, "        text: Text::{:?},", format.text)?;
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

    let mut dest_file = File::create(&dest_path)?;

    writeln!(
        dest_file,
        "use crate::file_types_format::{{FileFormat, Open, Text, Source}};"
    )?;

    for category in categories {
        transcribe_file_ext(&mut dest_file, category)?;
    }

    Ok(())
}

const DIR_STD_DIRS_ROOT: &str = "resources/osh-dir-std/mod/";

fn read_dir_stds() -> Result<HashMap<String, dir_std::DirStandard>, Box<dyn Error>> {
    let mut dir_stds = HashMap::new();
    for f in fs::read_dir(DIR_STD_DIRS_ROOT)? {
        let f = f?;

        if !f.file_type()?.is_dir() {
            continue;
        }

        let def_file = fs::canonicalize(f.path().join("definition.csv"))?;
        println!("cargo:rerun-if-changed={}", def_file.display());
        let dir_standard = dir_std::DirStandard::from_csv_file(&def_file)?;
        dir_stds.insert(f.file_name().to_string_lossy().to_string(), dir_standard);
    }

    Ok(dir_stds)
}

fn transcribe_dir_stds() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("dir_stds.rs");

    let mut dir_stds_out = File::create(&dest_path)?;

    writeln!(
        &mut dir_stds_out,
        r##"
    use std::collections::HashMap;
    use once_cell::sync::Lazy;
    use regex::Regex;
    use crate::dir_std;
    
    "##,
    )?;
    writeln!(
        &mut dir_stds_out,
        r##"pub static STDS: Lazy<HashMap<&'static str, dir_std::format::DirStd>> = Lazy::new(|| {});
"##,
        read_dir_stds()?.init_code()
    )?;

    Ok(())
}

fn main() {
    if let Err(err) = transcribe_file_exts() {
        println!("error running transcribe_file_exts(): {}", err);
        process::exit(1);
    }
    if let Err(err) = transcribe_dir_stds() {
        println!("error running transcribe_dir_stds(): {}", err);
        process::exit(2);
    }
}
