use std::fs;

use anyhow::{bail, Context};
use fs_extra::dir::copy;
use semver::Version;

use crate::{manifest::Manifest, package::PackageManifest};

pub fn package(name: String, version: Version) -> anyhow::Result<()> {
    let manifest = Manifest::get_or_create()?;

    let root_dir = manifest
        .get_package_path(&name)
        .context("resolving package name into source path")?;

    let package_dir = root_dir.join(version.to_string());

    if !package_dir.exists() {
        bail!(
            "{} version {} not found (searched at {:?})",
            name,
            version,
            package_dir
        );
    }

    let raw = fs::read_to_string(package_dir.join("typst.toml"))
        .context(format!("fetch manifest from {}:{:?}", name, version))?;
    let manifest: PackageManifest = toml::from_str(&raw).context("manifest corrupted")?;

    if name != manifest.package.name {
        bail!(
            "package name ({}) does not match name in manifest ({})",
            name,
            manifest.package.name
        );
    }
    if version != manifest.package.version {
        bail!(
            "package directory name ({}) does not match version in manifest ({})",
            version,
            manifest.package.version
        );
    }
    let entrypoint = package_dir.join(&manifest.package.entrypoint);
    if !entrypoint.exists() {
        bail!(
            "package entry point is missing, looked for {:?}",
            manifest.package.entrypoint
        );
    }

    let install_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join(&format!(
            "typst/packages/{}-{}",
            manifest.package.name, manifest.package.version
        ));

    fs::create_dir_all(&install_dir)
        .context("creating typst package bundle directory in /local")?;
    let mut options = fs_extra::dir::CopyOptions {
        skip_exist: true,
        content_only: true,
        ..Default::default()
    };
    if install_dir.exists() {
        options.overwrite = true;
    }
    copy(&package_dir, &install_dir, &options)?;

    Ok(())
}

// when no version was specified install/update all package entries
pub fn all_packages(name: String) -> anyhow::Result<()> {
    let manifest = Manifest::get_or_create()?;

    let root_dir = manifest
        .get_package_path(&name)
        .context("resolving package name into source path")?;

    let available_versions: Vec<semver::Version> = fs::read_dir(&root_dir)?
        .filter_map(Result::ok)
        .map(|e| semver::Version::parse(&e.file_name().to_string_lossy()))
        .filter_map(Result::ok)
        .collect();

    for version in available_versions {
        let package_dir = root_dir.join(version.to_string());

        if !package_dir.exists() {
            eprintln!(
                "{} version {} not found (searched at {:?})",
                name, version, package_dir
            );
            continue;
        }

        let raw = fs::read_to_string(package_dir.join("typst.toml"))
            .context(format!("fetch manifest from {}:{:?}", name, version))?;
        let manifest: PackageManifest = toml::from_str(&raw).context("manifest corrupted")?;

        if name != manifest.package.name {
            eprintln!(
                "package name ({}) does not match name in manifest ({}:{})",
                name, manifest.package.name, manifest.package.version
            );
            continue;
        }
        if version != manifest.package.version {
            eprintln!(
                "package directory name ({}) does not match version in manifest ({})",
                version, manifest.package.version
            );
            continue;
        }
        let entrypoint = package_dir.join(&manifest.package.entrypoint);
        if !entrypoint.exists() {
            eprintln!(
                "package entry point is missing, looked for {:?}",
                manifest.package.entrypoint
            );
            continue;
        }

        let install_dir = dirs::data_dir()
            .expect("failed to locate data directory")
            .join(&format!(
                "typst/packages/{}-{}",
                manifest.package.name, manifest.package.version
            ));

        fs::create_dir_all(&install_dir)
            .context("creating typst package bundle directory in /local")?;
        let options = fs_extra::dir::CopyOptions {
            skip_exist: true,
            content_only: true,
            ..Default::default()
        };
        copy(&package_dir, &install_dir, &options)?;
    }

    Ok(())
}
