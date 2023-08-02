use std::{fs, path::PathBuf};

use anyhow::Context;
use semver::Version;

use crate::Manifest;

pub fn bundle(name: String, version: Option<Version>) -> anyhow::Result<()> {
    let mut manifest = Manifest::get_or_create()?;

    let root_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages");

    if let Some(version) = version {
        fs::remove_dir_all(root_dir.join(format!("{}-{}", name, version)))
            .context(format!("cleaning \"{}\" package version {}", name, version))?;

        return Ok(());
    }

    let package_version_dirs: Vec<PathBuf> = fs::read_dir(&root_dir)?
        .filter_map(Result::ok)
        .map(|e| e.file_name().into_string())
        .filter_map(Result::ok)
        .filter(|s| s.contains(&name))
        .map(|s| root_dir.join(&s))
        .collect();

    for dir in package_version_dirs {
        fs::remove_dir_all(dir)
            .context(format!("cleaning all versions of package \"{}\"", name))?;
    }

    manifest.unregister(&name)
}

pub fn all() -> anyhow::Result<()> {
    let typst_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst");

    fs::remove_dir_all(typst_dir).context("cleaning all packages")
}
