// SPDX-FileCopyrightText: 2022 Robin Vobruba <hoijui.quaero@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{borrow::Cow, collections::HashMap};

pub trait Codify {
    fn init_code(&self) -> Cow<'static, str>;
}

// impl<T: AsRef<str>> Codify for T {
//     fn init_code(&self) -> Cow<'static, str> {
//         Cow::Owned(format!(r##""{}""##, self.as_ref()))
//     }
// }

impl Codify for String {
    fn init_code(&self) -> Cow<'static, str> {
        // NOTE: This codifies the String into a &'static str!
        Cow::Owned(format!(r##""{}""##, self))
    }
}

impl Codify for &str {
    fn init_code(&self) -> Cow<'static, str> {
        Cow::Owned(format!(r##""{}""##, self))
    }
}

impl<T> Codify for Vec<T>
    where T: Codify
{
    fn init_code(&self) -> Cow<'static, str> {
        let mut parts = vec!();

        parts.push("vec!(\n".to_string());
        for entry in self {
            parts.push(format!("{},\n", entry.init_code()));
        }
        parts.push(")".to_string());

        Cow::Owned(parts.concat())
    }
}

impl<K, V> Codify for HashMap<K, V>
where
    K: Codify,
    V: Codify
{
    fn init_code(&self) -> Cow<'static, str> {
        let mut parts = vec!();

        parts.push("{{\nlet mut map = HashMap::new();\n".to_string());
        for (key, val) in self {
            parts.push(format!(r##"map.insert({}, {});
"##, key.init_code(), val.init_code()));
        }
        parts.push("map\n}}".to_string());

        Cow::Owned(parts.concat())
    }
}