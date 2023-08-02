use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use fs_extra::dir::copy;
use url::Url;

use crate::manifest::Manifest;
use crate::package::PackageSpec;

pub fn package(raw_path: String) -> anyhow::Result<()> {
    if let Ok(url) = Url::from_str(&raw_path) {
        // TODO: clone from git and install
    }

    let path = PathBuf::from(raw_path);
    let pkg_manifest = path.join("typst.toml");

    if !pkg_manifest.exists() {
        anyhow::bail!(
            "package does not have a valid typst.toml manifest, searched at {:?}",
            pkg_manifest
        );
    }

    let spec: PackageSpec = toml::from_str(&fs::read_to_string(&pkg_manifest)?)?;

    // TODO: check if manifest is valid

    // TODO: fetch from manifest instead
    // remove the name of the folder from the path completely and get everything
    // from the manifest. then place the folder with the newly formatted name
    // {name}-{version} into /local/. you can also do validation when you have
    // the package spec.
    //
    // pkg_manifest.pop();
    // let pkg = pkg_manifest
    //     .file_name()
    //     .context("path terminated in ..")?
    //     .to_string_lossy();
    // let mut pkg = pkg.split("-");
    // let name = pkg.next().unwrap();
    // let version = pkg.next().unwrap();

    let subdir = format!(
        "typst/packages/local/{}-{}",
        spec.package.name, spec.package.version
    );

    if let Some(data_dir) = dirs::data_dir() {
        let dir = data_dir.join(&subdir);

        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }

        let options = fs_extra::dir::CopyOptions {
            copy_inside: true,
            ..Default::default()
        };
        copy(&path, dir, &options)?;
    }

    Ok(())
}
