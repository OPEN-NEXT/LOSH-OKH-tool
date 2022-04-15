// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{collections::HashMap, path::Path};

use relative_path::RelativePath;

/// Indicates out which relative paths of all dirs and files in a project
/// are covered by what parts of a specific dir standard.
// type Coverage<'a> = HashMap<&'static super::format::Rec<'static>, Vec<&'a RelativePath>>;
struct Coverage<'a>(HashMap<&'static super::format::Rec<'static>, Vec<&'a RelativePath>>);

impl<'a> Coverage<'a> {
    /// Given a set of the relative paths of all dirs and files in a project,
    /// figures out which of them are covered by what parts
    /// of a given dir standard.
    pub fn check<'b, T, S>(dirs_and_files: T, std: &'static super::format::DirStd) -> Coverage<'b>
    where
        T: IntoIterator<Item = &'b S> + Copy,
        // T::Item: &'a S,
        S: AsRef<RelativePath> + 'b,

        // A: Iterator<Item = &'a S1>,
        // S1: AsRef<OsStr> + 'a,
    {
        let mut rec_ratings = Coverage(HashMap::new());
        for record in &std.records {
            for dir_or_file in dirs_and_files.into_iter() {
                if record.regex.is_match(dir_or_file.as_ref().as_str()) {
                    rec_ratings
                        .0
                        .entry(record)
                        .or_insert_with(Vec::new)
                        .push(dir_or_file.as_ref());
                }
            }
        }
        rec_ratings
    }

    /// Calculates how much the input listing adheres to the input dir standard.
    /// 0.0 means not at all, 1.0 means totally/fully.
    pub fn rate(&self) -> f32 {
        let mut rating = 0.0;
        for (record, paths) in &self.0 {
            if paths.len() > 0 {
                rating += record.indicativeness;
            }
        }
        rating
    }

    /// Returns a list of the identified module(parts) directories.
    /// In addition to these,
    /// we should also consider all dirs that contain an okh.toml file.
    pub fn module_dirs(&self) -> Vec<&RelativePath> {
        let mut dirs = vec!();
        for (record, paths) in &self.0 {
            if record.module {
                for &path in paths {
                    dirs.push(path);
                }
            }
        }
        dirs
    }
}

/// Given a set of the relative paths of all dirs and files in a project,
/// for each of the known dir standards from
/// TODO Add URL to dir standards repo dir,
/// calculate how likely it seems
/// that the project is following this standard.
pub fn rate_listing<'a, T, S>(dirs_and_files: T) -> HashMap<&'static str, f32>
where
    // T: IntoIterator + Copy,
    // T::Item: AsRef<RelativePath>,
    T: IntoIterator<Item = &'a S> + Copy,
    S: AsRef<RelativePath> + 'a,
{
    let mut ratings = HashMap::new();
    for (std_name, std_records) in super::data::STDS.iter() {
        let std_coverage = Coverage::check(dirs_and_files, std_records);
        let rating = std_coverage.rate();
        ratings.insert(*std_name, rating);
    }
    ratings
}

/// Given a `proj_root`, for each of the known dir standards from
/// TODO Add URL to dir standards repo dir,
/// calculate how likely it seems
/// that the project is following this standard.
pub fn rate(proj_root: &Path) -> HashMap<&str, f32> {
    todo!();
}