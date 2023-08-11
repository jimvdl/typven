use std::{
    fs,
    path::{Path, PathBuf},
};

use semver::Version;
use serde::{Deserialize, Serialize};

pub struct Package {
    pub path: PathBuf,
    pub name: String,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package: PackageSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "package")]
pub struct PackageSpec {
    pub name: String,
    pub version: Version,
    pub entrypoint: PathBuf,
}

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
