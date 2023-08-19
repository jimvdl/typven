//! Package and package manifest, with helper functions
//!
//! Packages always live in folders named as `{name}/{version}`. The name and
//! version in the folder name and manifest must match.

use std::{
    fs,
    path::{Path, PathBuf},
};

use semver::Version;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

/// A collection of Typst files and assets that can be imported as a unit.
#[derive(Debug)]
pub struct Package {
    pub path: PathBuf,
    pub name: String,
    pub version: Version,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.version)
    }
}

/// The `typst.toml` package manifest that is required to be considered a
/// "valid" package.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package: PackageSpec,
}

/// The `[package]` specification with only the required fields.
///
/// The compiler requires every package to at least define:
/// - `name`: The package's identifier in its namespace.
/// - `version`: The package's version as a full major-minor-patch triple.
/// Package versioning should follow [SemVer].
/// - `entrypoint`: The path to the main Typst file that is evaluated when the
/// package is imported.
///
/// [SemVer]: https://semver.org/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "package")]
pub struct PackageSpec {
    pub name: String,
    pub version: Version,
    pub entrypoint: PathBuf,
}

/// Determines if the `path` directory contains a Typst package.
///
/// Only finds a package if:
/// - `typst.toml` manifest is present in the root directory.
/// - `typst.toml` contains the [required fields].
///
/// [required fields]: PackageSpec
pub fn is_package<P: AsRef<Path>>(path: &P) -> Option<Package> {
    let path = path.as_ref();

    fs::read_to_string(path.join("typst.toml"))
        .ok()
        .and_then(|s| toml::from_str(s.as_str()).ok())
        .map(|m: PackageManifest| Package {
            path: path.to_path_buf(),
            name: m.package.name,
            version: m.package.version,
        })
}

/// Searches the current `path` and every sub-directory (2 levels deep) for
/// valid packages. Internally uses [`is_package`] on each directory.
///
/// This function cannot fail -- it will simply not include directories that
/// fail to pass the [`is_package`] test and will only yield valid packages (if
/// any).
///
/// [`is_package`]: is_package
pub fn search<P: AsRef<Path>>(path: &P) -> Vec<Package> {
    WalkDir::new(&path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|e| e.path().is_dir().then(|| is_package(&e.path()))?)
        .collect()
}
