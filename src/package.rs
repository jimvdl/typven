use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package: PackageSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "package")]
pub struct PackageSpec {
    pub name: String,
    pub version: semver::Version,
    pub entrypoint: PathBuf,
}
