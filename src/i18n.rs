//! The texts of the plugin, in the language of the IDE: `tr!` and `trf!`, as in Atelier.
//!
//! The language is the one the IDE gave last (each frame, and when it asks what the plugin
//! brings): the threads of the plugin have no host to ask.

use atelier_plugin_api::Locales;
use std::sync::{LazyLock, RwLock};

static LOCALES: LazyLock<Locales> = LazyLock::new(|| {
    Locales::new(
        [
            ("en_US", include_str!("../locales/en_US.json")),
            ("fr_FR", include_str!("../locales/fr_FR.json")),
        ]
        .map(|(language, json)| {
            (
                language,
                atelier_plugin_api::serde_json::from_str(json).unwrap_or_default(),
            )
        }),
    )
});

static LANGUAGE: RwLock<String> = RwLock::new(String::new());

pub fn set_language(language: &str) {
    if *LANGUAGE.read().unwrap() != language {
        *LANGUAGE.write().unwrap() = language.to_string();
    }
}

/// The language of the texts: French in the tests, whose texts are French.
fn language() -> String {
    let language = LANGUAGE.read().unwrap();
    if language.is_empty() && cfg!(test) {
        "fr_FR".into()
    } else {
        language.clone()
    }
}

pub fn text(key: &'static str) -> &'static str {
    LOCALES.get(&language(), key)
}

pub fn format(key: &'static str, values: &[(&str, &dyn std::fmt::Display)]) -> String {
    LOCALES.format(&language(), key, values)
}

macro_rules! tr {
    ($key:literal $(,)?) => {
        $crate::i18n::text($key)
    };
}

macro_rules! trf {
    ($key:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::i18n::format($key, &[$((stringify!($name), &$value as &dyn std::fmt::Display)),*])
    };
}

/// The key of a text, translated when it is read: for tables of texts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Text(pub &'static str);

impl Text {
    pub fn get(self) -> &'static str {
        text(self.0)
    }
}
