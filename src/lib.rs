// SPDX-FileCopyrightText: 2021 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod conversion;
mod formats;
mod license;
mod logger;
mod macros;
mod oxrl;
mod validation;

use git_version::git_version;

pub const VERSION: &str = git_version!(cargo_prefix = "", fallback = "unknown");
