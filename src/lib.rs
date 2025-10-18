//! notox is a tool to clean file names.
//!
//! ## Usage as a binary
//!
//! ```shell
//! # install notox
//! cargo install notox
//! # then use it (dry-run by default)
//! notox .
//! # to rename the file
//! notox -d .
//! ```
//!
//! ## Usage as a library
//!
//! ```rust
//! use std::collections::HashSet;
//! use std::path::PathBuf;
//! use notox::{notox, JsonOutput, Notox, NotoxArgs, Output};
//!
//! let paths: HashSet<PathBuf> = HashSet::from(["README.md".into(), "Cargo.toml".into()]);
//! let notox_args = NotoxArgs {
//!     dry_run: true, // change here
//!     output: Output::JsonOutput {
//!         json: JsonOutput::JsonDefault,
//!         pretty: false,
//!     },
//!     // output: Output::Quiet
//! };
//! // as rust struct
//! let res = Notox::new(&notox_args).run(&paths);
//! // as function
//! let res = notox(&notox_args, &paths);
//! ```
//!
//! Coverage is available at [https://n4n5.dev/notox/coverage/](https://n4n5.dev/notox/coverage/)
//!

#![warn(clippy::all, rust_2018_idioms)]
#![deny(
    missing_docs,
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cargo
)]

use core::fmt;
use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

#[cfg(feature = "rayon")]
use rayon::{iter::Either, prelude::*};

/// Type of JSON output
#[cfg(feature = "serde")]
#[derive(Debug, Clone, PartialEq)]
pub enum JsonOutput {
    /// full json output
    JsonDefault,

    /// only errors in json output
    JsonOnlyError,
}

#[cfg(feature = "serde")]
impl fmt::Display for JsonOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonOutput::JsonDefault => write!(f, "json-default"),
            JsonOutput::JsonOnlyError => write!(f, "json-only-error"),
        }
    }
}

/// Type of output
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    /// default verbose output
    Default,

    /// quiet output
    Quiet,

    /// json output type and pretty print flag
    #[cfg(feature = "serde")]
    JsonOutput {
        /// which kind of json output to use
        json: JsonOutput,
        /// whether to pretty print the json output
        pretty: bool,
    },
}

impl Output {
    /// Check if the output is verbose
    pub fn is_verbose(&self) -> bool {
        matches!(self, Output::Default)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Options for the program
pub struct NotoxArgs {
    /// if true, the program will not rename files
    pub dry_run: bool,

    /// which kind of json output to use
    pub output: Output,
}

impl NotoxArgs {
    /// Create a new NotoxArgs instance with default values
    pub fn is_vervose(&self) -> bool {
        self.output.is_verbose()
    }
}

impl fmt::Display for NotoxArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't need to print output when
        // - it's Output::Quiet because it's quiet
        // - it's Output::JsonOutput because it's json
        // so, only one case is default
        write!(f, "NotoxArgs {{ dry_run: {} }}", self.dry_run)
    }
}

/// Contains information about a result of a single file
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PathChange {
    /// The path has not been changed
    Unchanged {
        /// The original path
        path: PathBuf,
    },
    /// The path has been changed
    Changed {
        /// The original path
        path: PathBuf,
        /// The modified path
        modified: PathBuf,
    },
    /// The path could not be changed
    ErrorRename {
        /// The original path
        path: PathBuf,
        /// The modified path
        modified: PathBuf,
        /// The error message
        error: String,
    },
    /// There was an error while processing the path
    Error {
        /// The original path
        path: PathBuf,
        /// The error message
        error: String,
    },
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PathChange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper {
            /// Path string
            path: String,
            /// Modified string
            modified: Option<String>,
            /// Error string
            error: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let path = PathBuf::from(helper.path);
        match (helper.modified, helper.error) {
            (None, None) => Ok(PathChange::Unchanged { path }),
            (Some(modified), None) => Ok(PathChange::Changed {
                path,
                modified: PathBuf::from(modified),
            }),
            (Some(modified), Some(error)) => Ok(PathChange::ErrorRename {
                path,
                modified: PathBuf::from(modified),
                error,
            }),
            (None, Some(error)) => Ok(PathChange::Error { path, error }),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for PathChange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("PathChange", 3)?;
        match self {
            PathChange::Unchanged { path } => {
                state.serialize_field("path", path)?;
                state.serialize_field("modified", &Option::<PathBuf>::None)?;
                state.serialize_field("error", &Option::<String>::None)?;
            }
            PathChange::Changed { path, modified } => {
                state.serialize_field("path", path)?;
                state.serialize_field("modified", &Some(modified))?;
                state.serialize_field("error", &Option::<String>::None)?;
            }
            PathChange::ErrorRename {
                path,
                modified,
                error,
            } => {
                state.serialize_field("path", path)?;
                state.serialize_field("modified", &Some(modified))?;
                state.serialize_field("error", &Some(error))?;
            }
            PathChange::Error { path, error } => {
                state.serialize_field("path", path)?;
                state.serialize_field("modified", &Option::<PathBuf>::None)?;
                state.serialize_field("error", &Some(error))?;
            }
        }
        state.end()
    }
}

/// Push a char to a string if a condition is true
#[inline(always)]
fn push_underscore_if(stri: &mut String, to_push: char, condition: bool) {
    if condition {
        stri.push(to_push);
    }
}

/// Check if a vector of bytes is similar to a char
#[inline(always)]
pub fn check_similar(curr_char: Option<char>, name_acc: &mut String, last_was_under: bool) -> bool {
    if let Some(one_char) = curr_char {
        match one_char {
            'A' | 'Ⓐ' | 'Ａ' | 'À' | 'Á' | 'Â' | 'Ầ' | 'Ấ' | 'Ẫ' | 'Ẩ' | 'Ã' | 'Ā' | 'Ă' | 'Ằ'
            | 'Ắ' | 'Ẵ' | 'Ẳ' | 'Ȧ' | 'Ǡ' | 'Ä' | 'Ǟ' | 'Ả' | 'Å' | 'Ǻ' | 'Ǎ' | 'Ȁ' | 'Ȃ' | 'Ạ'
            | 'Ậ' | 'Ặ' | 'Ḁ' | 'Ą' | 'Ⱥ' | 'Ɐ' => name_acc.push('A'),
            'Ꜳ' => name_acc.push_str("AA"),
            'Æ' | 'Ǽ' | 'Ǣ' => name_acc.push('A'),
            'Ꜵ' => name_acc.push_str("AO"),
            'Ꜷ' => name_acc.push_str("AU"),
            'Ꜹ' | 'Ꜻ' => name_acc.push_str("AV"),
            'Ꜽ' => name_acc.push_str("AY"),
            'B' | 'Ⓑ' | 'Ｂ' | 'Ḃ' | 'Ḅ' | 'Ḇ' | 'Ƀ' | 'Ƃ' | 'Ɓ' => name_acc.push('B'),
            'C' | 'Ⓒ' | 'Ｃ' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' | 'Ç' | 'Ḉ' | 'Ƈ' | 'Ȼ' | 'Ꜿ' => {
                name_acc.push('C')
            }
            'D' | 'Ⓓ' | 'Ｄ' | 'Ḋ' | 'Ď' | 'Ḍ' | 'Ḑ' | 'Ḓ' | 'Ḏ' | 'Đ' | 'Ƌ' | 'Ɗ' | 'Ɖ' | 'Ꝺ' => {
                name_acc.push('D')
            }
            'Ǳ' | 'Ǆ' => name_acc.push_str("DZ"),
            'ǲ' | 'ǅ' => name_acc.push_str("Dz"),
            'E' | 'Ⓔ' | 'Ｅ' | 'È' | 'É' | 'Ê' | 'Ề' | 'Ế' | 'Ễ' | 'Ể' | 'Ẽ' | 'Ē' | 'Ḕ' | 'Ḗ'
            | 'Ĕ' | 'Ė' | 'Ë' | 'Ẻ' | 'Ě' | 'Ȅ' | 'Ȇ' | 'Ẹ' | 'Ệ' | 'Ȩ' | 'Ḝ' | 'Ę' | 'Ḙ' | 'Ḛ'
            | 'Ɛ' | 'Ǝ' => name_acc.push('E'),
            'F' | 'Ⓕ' | 'Ｆ' | 'Ḟ' | 'Ƒ' | 'Ꝼ' => name_acc.push('F'),
            'G' | 'Ⓖ' | 'Ｇ' | 'Ǵ' | 'Ĝ' | 'Ḡ' | 'Ğ' | 'Ġ' | 'Ǧ' | 'Ģ' | 'Ǥ' | 'Ɠ' | 'Ꞡ' | 'Ᵹ'
            | 'Ꝿ' => name_acc.push('G'),
            'H' | 'Ⓗ' | 'Ｈ' | 'Ĥ' | 'Ḣ' | 'Ḧ' | 'Ȟ' | 'Ḥ' | 'Ḩ' | 'Ḫ' | 'Ħ' | 'Ⱨ' | 'Ⱶ' | 'Ɥ' => {
                name_acc.push('H')
            }
            'I' | 'Ⓘ' | 'Ｉ' | 'Ì' | 'Í' | 'Î' | 'Ĩ' | 'Ī' | 'Ĭ' | 'İ' | 'Ï' | 'Ḯ' | 'Ỉ' | 'Ǐ'
            | 'Ȉ' | 'Ȋ' | 'Ị' | 'Į' | 'Ḭ' | 'Ɨ' => name_acc.push('I'),
            'J' | 'Ⓙ' | 'Ｊ' | 'Ĵ' | 'Ɉ' => name_acc.push('J'),
            'K' | 'Ⓚ' | 'Ｋ' | 'Ḱ' | 'Ǩ' | 'Ḳ' | 'Ķ' | 'Ḵ' | 'Ƙ' | 'Ⱪ' | 'Ꝁ' | 'Ꝃ' | 'Ꝅ' | 'Ꞣ' => {
                name_acc.push('K')
            }
            'L' | 'Ⓛ' | 'Ｌ' | 'Ŀ' | 'Ĺ' | 'Ľ' | 'Ḷ' | 'Ḹ' | 'Ļ' | 'Ḽ' | 'Ḻ' | 'Ł' | 'Ƚ' | 'Ɫ'
            | 'Ⱡ' | 'Ꝉ' | 'Ꝇ' | 'Ꞁ' => name_acc.push('L'),
            'Ǉ' => name_acc.push_str("LJ"),
            'ǈ' => name_acc.push_str("Lj"),
            'M' | 'Ⓜ' | 'Ｍ' | 'Ḿ' | 'Ṁ' | 'Ṃ' | 'Ɱ' | 'Ɯ' => name_acc.push('M'),
            'N' | 'Ⓝ' | 'Ｎ' | 'Ǹ' | 'Ń' | 'Ñ' | 'Ṅ' | 'Ň' | 'Ṇ' | 'Ņ' | 'Ṋ' | 'Ṉ' | 'Ƞ' | 'Ɲ'
            | 'Ꞑ' | 'Ꞥ' => name_acc.push('N'),
            'Ǌ' => name_acc.push_str("NJ"),
            'ǋ' => name_acc.push_str("Nj"),
            'O' | 'Ⓞ' | 'Ｏ' | 'Ò' | 'Ó' | 'Ô' | 'Ồ' | 'Ố' | 'Ỗ' | 'Ổ' | 'Õ' | 'Ṍ' | 'Ȭ' | 'Ṏ'
            | 'Ō' | 'Ṑ' | 'Ṓ' | 'Ŏ' | 'Ȯ' | 'Ȱ' | 'Ö' | 'Ȫ' | 'Ỏ' | 'Ő' | 'Ǒ' | 'Ȍ' | 'Ȏ' | 'Ơ'
            | 'Ờ' | 'Ớ' | 'Ỡ' | 'Ở' | 'Ợ' | 'Ọ' | 'Ộ' | 'Ǫ' | 'Ǭ' | 'Ø' | 'Ǿ' | 'Ɔ' | 'Ɵ' | 'Ꝋ'
            | 'Ꝍ' => name_acc.push('O'),
            'Ƣ' => name_acc.push_str("OI"),
            'Ꝏ' => name_acc.push_str("OO"),
            'Ȣ' => name_acc.push_str("OU"),
            '\u{008C}' | 'Œ' => name_acc.push_str("OE"),
            '\u{009C}' | 'œ' => name_acc.push_str("oe"),
            'P' | 'Ⓟ' | 'Ｐ' | 'Ṕ' | 'Ṗ' | 'Ƥ' | 'Ᵽ' | 'Ꝑ' | 'Ꝓ' | 'Ꝕ' => {
                name_acc.push('P')
            }
            'Q' | 'Ⓠ' | 'Ｑ' | 'Ꝗ' | 'Ꝙ' | 'Ɋ' => name_acc.push('Q'),
            'R' | 'Ⓡ' | 'Ｒ' | 'Ŕ' | 'Ṙ' | 'Ř' | 'Ȑ' | 'Ȓ' | 'Ṛ' | 'Ṝ' | 'Ŗ' | 'Ṟ' | 'Ɍ' | 'Ɽ'
            | 'Ꝛ' | 'Ꞧ' | 'Ꞃ' => name_acc.push('R'),
            'S' | 'Ⓢ' | 'Ｓ' | 'ẞ' | 'Ś' | 'Ṥ' | 'Ŝ' | 'Ṡ' | 'Š' | 'Ṧ' | 'Ṣ' | 'Ṩ' | 'Ș' | 'Ş'
            | 'Ȿ' | 'Ꞩ' | 'Ꞅ' => name_acc.push('S'),
            'T' | 'Ⓣ' | 'Ｔ' | 'Ṫ' | 'Ť' | 'Ṭ' | 'Ț' | 'Ţ' | 'Ṱ' | 'Ṯ' | 'Ŧ' | 'Ƭ' | 'Ʈ' | 'Ⱦ'
            | 'Ꞇ' => name_acc.push('T'),
            'Ꜩ' => name_acc.push_str("TZ"),
            'U' | 'Ⓤ' | 'Ｕ' | 'Ù' | 'Ú' | 'Û' | 'Ũ' | 'Ṹ' | 'Ū' | 'Ṻ' | 'Ŭ' | 'Ü' | 'Ǜ' | 'Ǘ'
            | 'Ǖ' | 'Ǚ' | 'Ủ' | 'Ů' | 'Ű' | 'Ǔ' | 'Ȕ' | 'Ȗ' | 'Ư' | 'Ừ' | 'Ứ' | 'Ữ' | 'Ử' | 'Ự'
            | 'Ụ' | 'Ṳ' | 'Ų' | 'Ṷ' | 'Ṵ' | 'Ʉ' => name_acc.push('U'),
            'V' | 'Ⓥ' | 'Ｖ' | 'Ṽ' | 'Ṿ' | 'Ʋ' | 'Ꝟ' | 'Ʌ' => name_acc.push('V'),
            'Ꝡ' => name_acc.push_str("VY"),
            'W' | 'Ⓦ' | 'Ｗ' | 'Ẁ' | 'Ẃ' | 'Ŵ' | 'Ẇ' | 'Ẅ' | 'Ẉ' | 'Ⱳ' => {
                name_acc.push('W')
            }
            'X' | 'Ⓧ' | 'Ｘ' | 'Ẋ' | 'Ẍ' => name_acc.push('X'),
            'Y' | 'Ⓨ' | 'Ｙ' | 'Ỳ' | 'Ý' | 'Ŷ' | 'Ỹ' | 'Ȳ' | 'Ẏ' | 'Ÿ' | 'Ỷ' | 'Ỵ' | 'Ƴ' | 'Ɏ'
            | 'Ỿ' => name_acc.push('Y'),
            'Z' | 'Ⓩ' | 'Ｚ' | 'Ź' | 'Ẑ' | 'Ż' | 'Ž' | 'Ẓ' | 'Ẕ' | 'Ƶ' | 'Ȥ' | 'Ɀ' | 'Ⱬ' | 'Ꝣ' => {
                name_acc.push('Z')
            }
            'a' | 'ⓐ' | 'ａ' | 'ẚ' | 'à' | 'á' | 'â' | 'ầ' | 'ấ' | 'ẫ' | 'ẩ' | 'ã' | 'ā' | 'ă'
            | 'ằ' | 'ắ' | 'ẵ' | 'ẳ' | 'ȧ' | 'ǡ' | 'ä' | 'ǟ' | 'ả' | 'å' | 'ǻ' | 'ǎ' | 'ȁ' | 'ȃ'
            | 'ạ' | 'ậ' | 'ặ' | 'ḁ' | 'ą' | 'ⱥ' | 'ɐ' => name_acc.push('a'),
            'ꜳ' => name_acc.push_str("aa"),
            'æ' | 'ǽ' | 'ǣ' => name_acc.push('a'),
            'ꜵ' => name_acc.push_str("ao"),
            'ꜷ' => name_acc.push_str("au"),
            'ꜹ' | 'ꜻ' => name_acc.push_str("av"),
            'ꜽ' => name_acc.push_str("ay"),
            'b' | 'ⓑ' | 'ｂ' | 'ḃ' | 'ḅ' | 'ḇ' | 'ƀ' | 'ƃ' | 'ɓ' | 'þ' => {
                name_acc.push('b')
            }
            'c' | 'ⓒ' | 'ｃ' | 'ć' | 'ĉ' | 'ċ' | 'č' | 'ç' | 'ḉ' | 'ƈ' | 'ȼ' | 'ꜿ' | 'ↄ' => {
                name_acc.push('c')
            }
            'd' | 'ⓓ' | 'ｄ' | 'ḋ' | 'ď' | 'ḍ' | 'ḑ' | 'ḓ' | 'ḏ' | 'đ' | 'ƌ' | 'ɖ' | 'ɗ' | 'ꝺ' => {
                name_acc.push('d')
            }
            'ǳ' | 'ǆ' => name_acc.push_str("dz"),
            'e' | 'ⓔ' | 'ｅ' | 'è' | 'é' | 'ê' | 'ề' | 'ế' | 'ễ' | 'ể' | 'ẽ' | 'ē' | 'ḕ' | 'ḗ'
            | 'ĕ' | 'ė' | 'ë' | 'ẻ' | 'ě' | 'ȅ' | 'ȇ' | 'ẹ' | 'ệ' | 'ȩ' | 'ḝ' | 'ę' | 'ḙ' | 'ḛ'
            | 'ɇ' | 'ɛ' | 'ǝ' => name_acc.push('e'),
            'f' | 'ⓕ' | 'ｆ' | 'ḟ' | 'ƒ' | 'ꝼ' => name_acc.push('f'),
            'g' | 'ⓖ' | 'ｇ' | 'ǵ' | 'ĝ' | 'ḡ' | 'ğ' | 'ġ' | 'ǧ' | 'ģ' | 'ǥ' | 'ɠ' | 'ꞡ' | 'ᵹ'
            | 'ꝿ' => name_acc.push('g'),
            'h' | 'ⓗ' | 'ｈ' | 'ĥ' | 'ḣ' | 'ḧ' | 'ȟ' | 'ḥ' | 'ḩ' | 'ḫ' | 'ẖ' | 'ħ' | 'ⱨ' | 'ⱶ'
            | 'ɥ' => name_acc.push('h'),
            'ƕ' => name_acc.push_str("hv"),
            'i' | 'ⓘ' | 'ｉ' | 'ì' | 'í' | 'î' | 'ĩ' | 'ī' | 'ĭ' | 'ï' | 'ḯ' | 'ỉ' | 'ǐ' | 'ȉ'
            | 'ȋ' | 'ị' | 'į' | 'ḭ' | 'ɨ' | 'ı' => name_acc.push('i'),
            'j' | 'ⓙ' | 'ｊ' | 'ĵ' | 'ǰ' | 'ɉ' => name_acc.push('j'),
            'k' | 'ⓚ' | 'ｋ' | 'ḱ' | 'ǩ' | 'ḳ' | 'ķ' | 'ḵ' | 'ƙ' | 'ⱪ' | 'ꝁ' | 'ꝃ' | 'ꝅ' | 'ꞣ' => {
                name_acc.push('k')
            }
            'l' | 'ⓛ' | 'ｌ' | 'ŀ' | 'ĺ' | 'ľ' | 'ḷ' | 'ḹ' | 'ļ' | 'ḽ' | 'ḻ' | 'ſ' | 'ł' | 'ƚ'
            | 'ɫ' | 'ⱡ' | 'ꝉ' | 'ꞁ' | 'ꝇ' => name_acc.push('l'),
            'ǉ' => name_acc.push_str("lj"),
            'm' | 'ⓜ' | 'ｍ' | 'ḿ' | 'ṁ' | 'ṃ' | 'ɱ' | 'ɯ' => name_acc.push('m'),
            'n' | 'ⓝ' | 'ｎ' | 'ǹ' | 'ń' | 'ñ' | 'ṅ' | 'ň' | 'ṇ' | 'ņ' | 'ṋ' | 'ṉ' | 'ƞ' | 'ɲ'
            | 'ŉ' | 'ꞑ' | 'ꞥ' => name_acc.push('n'),
            'ǌ' => name_acc.push_str("nj"),
            'o' | 'ⓞ' | 'ｏ' | 'ò' | 'ó' | 'ô' | 'ồ' | 'ố' | 'ỗ' | 'ổ' | 'õ' | 'ṍ' | 'ȭ' | 'ṏ'
            | 'ō' | 'ṑ' | 'ṓ' | 'ŏ' | 'ȯ' | 'ȱ' | 'ö' | 'ȫ' | 'ỏ' | 'ő' | 'ǒ' | 'ȍ' | 'ȏ' | 'ơ'
            | 'ờ' | 'ớ' | 'ỡ' | 'ở' | 'ợ' | 'ọ' | 'ộ' | 'ǫ' | 'ǭ' | 'ø' | 'ǿ' | 'ɔ' | 'ꝋ' | 'ꝍ'
            | 'ɵ' => name_acc.push('o'),
            'ƣ' => name_acc.push_str("oi"),
            'ȣ' => name_acc.push_str("ou"),
            'ꝏ' => name_acc.push_str("oo"),
            'p' | 'ⓟ' | 'ｐ' | 'ṕ' | 'ṗ' | 'ƥ' | 'ᵽ' | 'ꝑ' | 'ꝓ' | 'ꝕ' => {
                name_acc.push('p')
            }
            'q' | 'ⓠ' | 'ｑ' | 'ɋ' | 'ꝗ' | 'ꝙ' => name_acc.push('q'),
            'r' | 'ⓡ' | 'ｒ' | 'ŕ' | 'ṙ' | 'ř' | 'ȑ' | 'ȓ' | 'ṛ' | 'ṝ' | 'ŗ' | 'ṟ' | 'ɍ' | 'ɽ'
            | 'ꝛ' | 'ꞧ' | 'ꞃ' => name_acc.push('r'),
            's' | 'ⓢ' | 'ｓ' | 'ß' | 'ś' | 'ṥ' | 'ŝ' | 'ṡ' | 'š' | 'ṧ' | 'ṣ' | 'ṩ' | 'ș' | 'ş'
            | 'ȿ' | 'ꞩ' | 'ꞅ' | 'ẛ' => name_acc.push('s'),
            't' | 'ⓣ' | 'ｔ' | 'ṫ' | 'ẗ' | 'ť' | 'ṭ' | 'ț' | 'ţ' | 'ṱ' | 'ṯ' | 'ŧ' | 'ƭ' | 'ʈ'
            | 'ⱦ' | 'ꞇ' => name_acc.push('t'),
            'ꜩ' => name_acc.push_str("tz"),
            'u' | 'ⓤ' | 'ｕ' | 'ù' | 'ú' | 'û' | 'ũ' | 'ṹ' | 'ū' | 'ṻ' | 'ŭ' | 'ü' | 'ǜ' | 'ǘ'
            | 'ǖ' | 'ǚ' | 'ủ' | 'ů' | 'ű' | 'ǔ' | 'ȕ' | 'ȗ' | 'ư' | 'ừ' | 'ứ' | 'ữ' | 'ử' | 'ự'
            | 'ụ' | 'ṳ' | 'ų' | 'ṷ' | 'ṵ' | 'ʉ' => name_acc.push('u'),
            'v' | 'ⓥ' | 'ｖ' | 'ṽ' | 'ṿ' | 'ʋ' | 'ꝟ' | 'ʌ' => name_acc.push('v'),
            'ꝡ' => name_acc.push_str("vy"),
            'w' | 'ⓦ' | 'ｗ' | 'ẁ' | 'ẃ' | 'ŵ' | 'ẇ' | 'ẅ' | 'ẘ' | 'ẉ' | 'ⱳ' => {
                name_acc.push('w')
            }
            'x' | 'ⓧ' | 'ｘ' | 'ẋ' | 'ẍ' => name_acc.push('x'),
            'y' | 'ⓨ' | 'ｙ' | 'ỳ' | 'ý' | 'ŷ' | 'ỹ' | 'ȳ' | 'ẏ' | 'ÿ' | 'ỷ' | 'ẙ' | 'ỵ' | 'ƴ'
            | 'ɏ' | 'ỿ' => name_acc.push('y'),
            'z' | 'ⓩ' | 'ｚ' | 'ź' | 'ẑ' | 'ż' | 'ž' | 'ẓ' | 'ẕ' | 'ƶ' | 'ȥ' | 'ɀ' | 'ⱬ' | 'ꝣ' => {
                name_acc.push('z')
            }
            '–' => {
                name_acc.push('-');
                return false;
            }
            '\u{0300}'..='\u{036F}' | '\u{1AB0}'..='\u{1AFF}' | '\u{1DC0}'..='\u{1DFF}' => {}
            _ => {
                if !last_was_under {
                    name_acc.push('_');
                }
                return true;
            }
        };
        return false;
    }
    false
}

/// Convert four bytes to a u32
#[inline(always)]
pub(crate) fn convert_four_to_u32(
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
) -> u32 {
    ((first_byte as u32 & 0b0000_0111) << 18)
        | ((second_byte as u32 & 0b0011_1111) << 12)
        | ((third_byte as u32 & 0b0011_1111) << 6)
        | (fourth_byte as u32 & 0b0011_1111)
}

/// Convert three bytes to a u32
#[inline(always)]
pub(crate) fn convert_three_to_u32(first_byte: u8, second_byte: u8, third_byte: u8) -> u32 {
    ((first_byte as u32 & 0b0001_1111) << 12)
        | ((second_byte as u32 & 0b0011_1111) << 6)
        | (third_byte as u32 & 0b0011_1111)
}

/// Convert two bytes to a u32
#[inline(always)]
pub(crate) fn convert_two_to_u32(first_byte: u8, second_byte: u8) -> u32 {
    ((first_byte as u32 & 0b0001_1111) << 6) | (second_byte as u32 & 0b0011_1111)
}

/// Clean a name
#[inline(always)]
fn clean_name(path: &OsStr, _options: &NotoxArgs) -> OsString {
    // for each byte of the path if it's not ascii, replace it with _
    let mut new_name = String::new();
    let mut vec_grapheme: [u8; 4] = [0; 4];
    let mut last_was_underscore = false;
    let mut idx_grapheme = 0;
    for byte in path.as_encoded_bytes().iter() {
        if idx_grapheme == 0 && *byte < 128 {
            match byte {
                0..=44 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                46 => {
                    new_name.push('.');
                    last_was_underscore = false;
                }
                47 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                58..=64 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                91..=96 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                123..=127 => {
                    push_underscore_if(&mut new_name, '_', !last_was_underscore);
                    last_was_underscore = true;
                }
                _ => {
                    new_name.push(*byte as char);
                    last_was_underscore = false;
                }
            }
            idx_grapheme = 0;
        } else {
            vec_grapheme[idx_grapheme] = *byte;
            idx_grapheme += 1;
            let first_byte = vec_grapheme[0];
            if first_byte >= 240 && idx_grapheme == 4 {
                // four bytes grapheme
                let curr_char = std::char::from_u32(convert_four_to_u32(
                    vec_grapheme[0],
                    vec_grapheme[1],
                    vec_grapheme[2],
                    vec_grapheme[3],
                ));
                last_was_underscore = check_similar(curr_char, &mut new_name, last_was_underscore);
                vec_grapheme = [0; 4];
                idx_grapheme = 0;
            } else if (224..240).contains(&first_byte) && idx_grapheme == 3 {
                // three bytes grapheme
                let curr_char = std::char::from_u32(convert_three_to_u32(
                    vec_grapheme[0],
                    vec_grapheme[1],
                    vec_grapheme[2],
                ));
                last_was_underscore = check_similar(curr_char, &mut new_name, last_was_underscore);
                vec_grapheme = [0; 4];
                idx_grapheme = 0;
            } else if (128..224).contains(&first_byte) && idx_grapheme == 2 {
                // two bytes grapheme
                let curr_char =
                    std::char::from_u32(convert_two_to_u32(vec_grapheme[0], vec_grapheme[1]));
                last_was_underscore = check_similar(curr_char, &mut new_name, last_was_underscore);
                vec_grapheme = [0; 4];
                idx_grapheme = 0;
            }
        }
    }
    OsString::from(new_name)
}

/// Clean a path
fn clean_path(file_path: &Path, options: &NotoxArgs) -> PathChange {
    let file_name = match file_path.file_name() {
        Some(name) => name,
        None => {
            return PathChange::Unchanged {
                path: file_path.to_path_buf(),
            };
        }
    };
    let cleaned_name = clean_name(file_name, options);
    if cleaned_name == file_name {
        return PathChange::Unchanged {
            path: file_path.to_path_buf(),
        };
    }
    let cleaned_path = file_path.with_file_name(cleaned_name);
    if options.dry_run {
        return PathChange::ErrorRename {
            path: file_path.to_path_buf(),
            modified: cleaned_path,
            error: "dry-run".to_string(),
        };
    }
    match std::fs::rename(file_path, &cleaned_path) {
        Ok(_) => PathChange::Changed {
            path: file_path.to_path_buf(),
            modified: cleaned_path,
        },
        Err(rename_error) => PathChange::ErrorRename {
            path: file_path.to_path_buf(),
            modified: cleaned_path,
            error: rename_error.to_string(),
        },
    }
}

/// Clean a directory
fn clean_directory(dir_path: &Path, options: &NotoxArgs) -> Vec<PathChange> {
    let mut dir_path = dir_path.to_path_buf();
    let mut result_vec = Vec::new();
    let res_dir = clean_path(&dir_path, options);
    if let PathChange::Changed { modified, .. } = &res_dir {
        dir_path = modified.clone();
    }
    result_vec.push(res_dir);
    if let Ok(entries) = std::fs::read_dir(&dir_path) {
        let ok_entries = {
            #[cfg(feature = "rayon")]
            {
                use std::fs::DirEntry;
                let (ok_entries, error_entries): (Vec<_>, Vec<_>) = entries
                    .collect::<Vec<Result<DirEntry, std::io::Error>>>()
                    .into_par_iter()
                    .partition_map(|x| match x {
                        Ok(entry) => Either::Left(entry),
                        Err(e) => Either::Right(e),
                    });
                error_entries.into_iter().for_each(|e| {
                    result_vec.push(PathChange::Error {
                        path: dir_path.clone(),
                        error: format!("Error reading dir entry of directory {}", e),
                    })
                });
                ok_entries
            }
            #[cfg(not(feature = "rayon"))]
            {
                let mut ok_entries = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(e) => ok_entries.push(e),
                        Err(e) => result_vec.push(PathChange::Error {
                            path: dir_path.clone(),
                            error: format!("Error reading dir entry of directory {}", e),
                        }),
                    }
                }
                ok_entries
            }
        };
        #[cfg(feature = "rayon")]
        let iter = ok_entries.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = ok_entries.iter();
        let mapped = iter
            .map(|entry| {
                let file_path = entry.path();
                let is_entry_directory = match entry.file_type() {
                    Ok(file_type) => file_type.is_dir(),
                    Err(_) => false,
                };
                if is_entry_directory {
                    clean_directory(&file_path, options)
                } else {
                    let res = clean_path(&file_path, options);
                    vec![res]
                }
            })
            .flatten()
            .collect::<Vec<PathChange>>();
        result_vec.extend(mapped);
    } else {
        result_vec.push(PathChange::Error {
            path: dir_path,
            error: "Error while reading directory".to_string(),
        });
    }
    result_vec
}

/// Get the path of a directory
#[inline(always)]
fn get_path_of_dir(dir_path: &str) -> HashSet<PathBuf> {
    match std::fs::read_dir(dir_path) {
        Ok(dir_entries) => dir_entries
            .into_iter()
            .filter_map(Result::ok)
            .map(|e| e.path().to_path_buf())
            .collect(),
        Err(_) => HashSet::new(),
    }
}

/// Show the version
#[inline(always)]
fn show_version() {
    /// Version of the program
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    /// Authors of the program
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    println!("notox {} by {}", &VERSION, &AUTHORS)
}

/// Parse the arguments and return the options and the paths to check
/// # Errors
/// Return an error if the path is not found
pub fn parse_args(args: &[String]) -> Result<(NotoxArgs, HashSet<PathBuf>), i32> {
    let mut dry_run = true;
    let mut output = Output::Default;
    let mut path_to_check: HashSet<PathBuf> = HashSet::new();
    for one_arg in &args[1..] {
        if one_arg == "-d" || one_arg == "--do" {
            dry_run = false;
        } else if one_arg == "-h" || one_arg == "--help" {
            println!("Usage: notox [options] [path]");
            show_version();
            println!("Options:");
            println!("  -d, --do          Do the renaming");
            println!("  -h, --help        Show this help message");
            println!("  -v, --version     Show the version");
            println!("  -p, --json-pretty Print the result in JSON format (pretty)");
            println!("  -e, --json-error  Print only the errors in JSON format");
            println!("  -j, --json        Print the result in JSON format");
            println!("  -q, --quiet       Do not print anything");
            return Err(1);
        } else if one_arg == "-v" || one_arg == "--version" {
            show_version();
            return Err(1);
        } else if one_arg == "-p" || one_arg == "--json-pretty" {
            #[cfg(feature = "serde")]
            {
                output = match output {
                    Output::JsonOutput {
                        json: json_output,
                        pretty: _,
                    } => Output::JsonOutput {
                        json: json_output,
                        pretty: true,
                    },
                    _ => Output::JsonOutput {
                        json: JsonOutput::JsonDefault,
                        pretty: true,
                    },
                };
            }
            #[cfg(not(feature = "serde"))]
            {
                println!("JSON output is not available, please use a notox version with the 'serde' feature.");
                return Err(2);
            }
        } else if one_arg == "-e" || one_arg == "--json-error" {
            #[cfg(feature = "serde")]
            {
                output = match output {
                    Output::JsonOutput { json: _, pretty } => Output::JsonOutput {
                        json: JsonOutput::JsonOnlyError,
                        pretty,
                    },
                    _ => Output::JsonOutput {
                        json: JsonOutput::JsonOnlyError,
                        pretty: false,
                    },
                };
            }
            #[cfg(not(feature = "serde"))]
            {
                println!("JSON output is not available, please use a notox version with the 'serde' feature.");
                return Err(2);
            }
        } else if one_arg == "-j" || one_arg == "--json" {
            #[cfg(feature = "serde")]
            {
                output = match output {
                    Output::JsonOutput { json: _, pretty } => Output::JsonOutput {
                        json: JsonOutput::JsonDefault,
                        pretty,
                    },
                    _ => Output::JsonOutput {
                        json: JsonOutput::JsonDefault,
                        pretty: false,
                    },
                };
            }
            #[cfg(not(feature = "serde"))]
            {
                println!("JSON output is not available, please use a notox version with the 'serde' feature.");
                return Err(2);
            }
        } else if one_arg == "-q" || one_arg == "--quiet" {
            output = Output::Quiet;
        } else if one_arg == "*" {
            // should not happen with most shells
            let paths = get_path_of_dir(".");
            path_to_check.extend(paths);
        } else if std::fs::metadata(one_arg).is_ok() {
            path_to_check.insert(PathBuf::from(one_arg));
        } else if output.is_verbose() {
            println!("Cannot find path: {}", one_arg);
        }
    }
    if path_to_check.is_empty() {
        let paths = get_path_of_dir(".");
        path_to_check.extend(paths);
    }
    Ok((NotoxArgs { dry_run, output }, path_to_check))
}

/// Print the output of the program conforming to the options
/// # Errors
/// Return an error if the output cannot be serialized
pub fn print_output(options: &NotoxArgs, final_res: Vec<PathChange>) -> Result<(), i32> {
    match &options.output {
        Output::Default => {
            let len = final_res.len();
            for one_change in final_res {
                match one_change {
                    PathChange::Unchanged { .. } => {}
                    PathChange::Changed { path, modified } => {
                        println!("{} -> {}", path.display(), modified.display());
                    }
                    PathChange::Error { path, error } => {
                        println!("{} : {}", path.display(), error);
                    }
                    PathChange::ErrorRename {
                        path,
                        modified,
                        error,
                    } => {
                        println!("{} -> {} : {}", path.display(), modified.display(), error);
                    }
                }
            }
            if len == 1 {
                println!("{} file checked", len);
            } else {
                println!("{} files checked", len);
            }
        }
        #[cfg(feature = "serde")]
        Output::JsonOutput {
            json: json_output,
            pretty: json_pretty,
        } => {
            let vec_to_json = match json_output {
                JsonOutput::JsonDefault => final_res,
                JsonOutput::JsonOnlyError => {
                    let mut vec_to_json: Vec<PathChange> = Vec::new();
                    for one_change in final_res {
                        match one_change {
                            PathChange::Unchanged { .. } => {}
                            PathChange::Changed { .. } => {}
                            one_res @ PathChange::Error { .. } => {
                                vec_to_json.push(one_res);
                            }
                            one_res @ PathChange::ErrorRename { .. } => {
                                vec_to_json.push(one_res);
                            }
                        }
                    }
                    vec_to_json
                }
            };
            let json_string = match json_pretty {
                true => serde_json::to_string_pretty(&vec_to_json),
                false => serde_json::to_string(&vec_to_json),
            };
            match json_string {
                Ok(stringed) => println!("{}", stringed),
                Err(_) => {
                    println!(r#"{{"error": "Cannot serialize result"}}"#);
                    return Err(2);
                }
            }
        }
        Output::Quiet => {}
    }
    Ok(())
}

/// Do the program, return the Vector of result
pub fn notox(notox_args: &NotoxArgs, paths_to_check: &HashSet<PathBuf>) -> Vec<PathChange> {
    Notox::new(notox_args).run(paths_to_check)
}

/// Notox struct
pub struct Notox {
    /// Options
    notox_args: NotoxArgs,
}

impl Notox {
    /// Create a new Notox instance
    pub fn new(notox_args: &NotoxArgs) -> Notox {
        Notox {
            notox_args: notox_args.clone(),
        }
    }

    /// Run from args
    /// # Errors
    /// Returns error if parse_args fails
    pub fn run_from_args(args: &[String]) -> Result<Vec<PathChange>, i32> {
        match parse_args(args) {
            Ok((notox_args, paths)) => Ok(Notox::new(&notox_args).run(&paths)),
            Err(code) => Err(code),
        }
    }

    /// Run main from args
    pub fn run_main_from_args(args: &[String]) -> i32 {
        match parse_args(args) {
            Ok((notox_args, paths)) => Notox::new(&notox_args).run_and_print(&paths),
            Err(code) => code,
        }
    }

    /// Run the Notox instance
    pub fn run(&self, paths_to_check: &HashSet<PathBuf>) -> Vec<PathChange> {
        if self.notox_args.is_vervose() {
            println!("Running with options: {}", &self.notox_args);
        }
        #[cfg(feature = "rayon")]
        let iter = paths_to_check.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = paths_to_check.iter();

        let results = iter
            .map(|one_path| {
                if self.notox_args.is_vervose() {
                    println!("Checking: {}", one_path.display());
                }
                match one_path.is_dir() {
                    true => clean_directory(one_path, &self.notox_args),
                    false => {
                        let one_cleaned = clean_path(one_path, &self.notox_args);
                        vec![one_cleaned]
                    }
                }
            })
            .flatten();
        results.collect::<Vec<PathChange>>()
    }

    /// Run the Notox instance and print the output
    pub fn run_and_print(self, path_to_check: &HashSet<PathBuf>) -> i32 {
        let final_res = self.run(path_to_check);
        match print_output(&self.notox_args, final_res) {
            Ok(_) => 0,
            Err(code) => code,
        }
    }
}
