// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use lingua::{Language, LanguageDetectorBuilder};

pub fn identify(text: &str) -> Option<String> {
    let detector = LanguageDetectorBuilder::from_all_languages().build();

    let detected_language: Option<Language> = detector.detect_language_of(text);
    detected_language.map(|lang| lang.iso_code_639_1().to_string())
}
