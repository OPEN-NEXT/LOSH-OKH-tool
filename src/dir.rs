// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use regex::Regex;
use walkdir::WalkDir;

pub fn walker<P>(input_path: P, recursive: bool) -> WalkDir
where
    P: AsRef<Path>,
{
    let mut walker = WalkDir::new(input_path);
    walker = walker.min_depth(1);
    if !recursive {
        walker = walker.max_depth(1);
    }
    walker
}

pub fn iter_exts(walker: WalkDir, ext_matcher: &'_ Regex) -> impl '_ + Iterator<Item = PathBuf> {
    walker
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.is_file()
                .then_some(path)
                .and_then(|path_ref| path_ref.extension())
                .and_then(|part| part.to_str())
                .filter(|ext_utf8| ext_matcher.is_match(ext_utf8))
                .is_some()
        })
        .map(|entry| entry.path().to_path_buf())
}

pub fn scan<P>(
    root_dir: P,
    recursive: bool,
    filters: &[&Regex],
    path_part_extractor: fn(&Path) -> Option<&OsStr>,
) -> Vec<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut matching = vec![];
    for _ in 0..filters.len() {
        matching.push(vec![]);
    }
    matching.len();
    walker(&root_dir, recursive)
        .into_iter()
        .filter_map(Result::ok)
        .for_each(|entry| {
            let path = entry.path().strip_prefix(&root_dir).unwrap();
            if let Some(part_utf8) = path
                .is_file()
                .then_some(path)
                .and_then(|path_ref| path_part_extractor(path_ref))
                .and_then(|part| part.to_str())
            {
                filters.iter().enumerate().for_each(|(i, flt)| {
                    if flt.is_match(part_utf8) {
                        matching.get_mut(i).unwrap().push(path.to_path_buf());
                    }
                });
            }
        });
    matching
}
