// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::File;
use std::path::Path;

use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, SharedLogger, TermLogger, TerminalMode,
    WriteLogger,
};

pub fn init(file: Option<&Path>, level: (LevelFilter, LevelFilter)) {
    let mut loggers: Vec<Box<(dyn SharedLogger + 'static)>> = vec![TermLogger::new(
        // LevelFilter::Info,
        level.0,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];
    if let Some(file_path) = file {
        loggers.push(WriteLogger::new(
            level.1,
            Config::default(),
            File::create(file_path).unwrap(),
        ));
    };
    CombinedLogger::init(loggers).unwrap();
    log::debug!("Logging activated.");
    if let Some(file_path) = file {
        log::info!("Logging to file '{:?}'.", file_path);
    }
}
