use std::env::current_dir;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use url::ParseError;
use url::Url;

pub const DUMMY_SPECIFIER: &str = "<unknown>";

/// Error indicating the reason resolving a module specifier failed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModuleResolutionError {
    InvalidUrl(ParseError),
    InvalidBaseUrl(ParseError),
    InvalidPath(PathBuf),
    ImportPrefixMissing(String, Option<String>),
}
use ModuleResolutionError::*;

use crate::normalize_path;

impl Error for ModuleResolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InvalidUrl(ref err) | InvalidBaseUrl(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ModuleResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidUrl(ref err) => write!(f, "invalid URL: {}", err),
            InvalidBaseUrl(ref err) => {
                write!(f, "invalid base URL for relative import: {}", err)
            }
            InvalidPath(ref path) => write!(f, "invalid module path: {:?}", path),
            ImportPrefixMissing(ref specifier, ref maybe_referrer) => write!(
                f,
                "Relative import path \"{}\" not prefixed with / or ./ or ../{}",
                specifier,
                match maybe_referrer {
                    Some(referrer) => format!(" from \"{}\"", referrer),
                    None => String::new(),
                }
            ),
        }
    }
}

/// Resolved module specifier
pub type ModuleSpecifier = Url;

/// Resolves module using this algorithm:
/// <https://html.spec.whatwg.org/multipage/webappapis.html#resolve-a-module-specifier>
pub fn resolve_import(
    specifier: &str,
    base: &str,
) -> Result<ModuleSpecifier, ModuleResolutionError> {
    let url = match Url::parse(specifier) {
        // 1. Apply the URL parser to specifier.
        //    If the result is not failure, return he result.
        Ok(url) => url,

        // 2. If specifier does not start with the character U+002F SOLIDUS (/),
        //    the two-character sequence U+002E FULL STOP, U+002F SOLIDUS (./),
        //    or the three-character sequence U+002E FULL STOP, U+002E FULL STOP,
        //    U+002F SOLIDUS (../), return failure.
        Err(ParseError::RelativeUrlWithoutBase)
            if !(specifier.starts_with('/')
                || specifier.starts_with("./")
                || specifier.starts_with("../")) =>
        {
            let maybe_referrer = if base.is_empty() {
                None
            } else {
                Some(base.to_string())
            };
            return Err(ImportPrefixMissing(specifier.to_string(), maybe_referrer));
        }

        // 3. Return the result of applying the URL parser to specifier with base
        //    URL as the base URL.
        Err(ParseError::RelativeUrlWithoutBase) => {
            let base = if base == DUMMY_SPECIFIER {
                // Handle <unknown> case, happening under e.g. repl.
                // Use CWD for such case.

                // Forcefully join base to current dir.
                // Otherwise, later joining in Url would be interpreted in
                // the parent directory (appending trailing slash does not work)
                let path = current_dir().unwrap().join(base);
                Url::from_file_path(path).unwrap()
            } else {
                Url::parse(base).map_err(InvalidBaseUrl)?
            };
            base.join(specifier).map_err(InvalidUrl)?
        }

        // If parsing the specifier as a URL failed for a different reason than
        // it being relative, always return the original error. We don't want to
        // return `ImportPrefixMissing` or `InvalidBaseUrl` if the real
        // problem lies somewhere else.
        Err(err) => return Err(InvalidUrl(err)),
    };

    Ok(url)
}

/// Converts a string representing an absolute URL into a ModuleSpecifier.
pub fn resolve_url(url_str: &str) -> Result<ModuleSpecifier, ModuleResolutionError> {
    Url::parse(url_str).map_err(ModuleResolutionError::InvalidUrl)
}

/// Takes a string representing either an absolute URL or a file path,
/// as it may be passed to Cassie as a command line argument.
/// The string is interpreted as a URL if it starts with a valid URI scheme,
/// e.g. 'http:' or 'file:' or 'git+ssh:'. If not, it's interpreted as a
/// file path; if it is a relative path it's resolved relative to the current
/// working directory.
pub fn resolve_url_or_path(specifier: &str) -> Result<ModuleSpecifier, ModuleResolutionError> {
    if specifier_has_uri_scheme(specifier) {
        resolve_url(specifier)
    } else {
        resolve_path(specifier)
    }
}

/// Converts a string representing a relative or absolute path into a
/// ModuleSpecifier. A relative path is considered relative to the current
/// working directory.
pub fn resolve_path(path_str: &str) -> Result<ModuleSpecifier, ModuleResolutionError> {
    let path = current_dir().unwrap().join(path_str);
    let path = normalize_path(&path);
    Url::from_file_path(path.clone()).map_err(|()| ModuleResolutionError::InvalidPath(path))
}

/// Returns true if the input string starts with a sequence of characters
/// that could be a valid URI scheme, like 'https:', 'git+ssh:' or 'data:'.
///
/// According to RFC 3986 (https://tools.ietf.org/html/rfc3986#section-3.1),
/// a valid scheme has the following format:
///   scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
///
/// We additionally require the scheme to be at least 2 characters long,
/// because otherwise a windows path like c:/foo would be treated as a URL,
/// while no schemes with a one-letter name actually exist.
fn specifier_has_uri_scheme(specifier: &str) -> bool {
    let mut chars = specifier.chars();
    let mut len = 0usize;
    // THe first character must be a letter.
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => len += 1,
        _ => return false,
    }
    // Second and following characters must be either a letter, number,
    // plus sign, minus sign, or dot.
    loop {
        match chars.next() {
            Some(c) if c.is_ascii_alphanumeric() || "+-.".contains(c) => len += 1,
            Some(':') if len >= 2 => return true,
            _ => return false,
        }
    }
}
