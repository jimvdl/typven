use std::{env, fs};

use anyhow::{bail, Context};
use fs_extra::dir::copy;

use crate::package::{is_package, Package, self};

pub fn packages() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("accessing current working directory failed")?;
    // let cwd = std::path::Path::new("C:\\Users\\Jim\\Desktop\\typst-templates\\0.1.0");
    // let cwd = std::path::Path::new("C:\\Users\\Jim\\Desktop");

    if let Some(package) = is_package(&cwd) {
        return install(package);
    }

    let packages = package::search(&cwd);

    if packages.is_empty() {
        bail!("no valid packages found");
    }

    for package in packages {
        install(package)?;
    }

    Ok(())
}

fn install(package: Package) -> anyhow::Result<()> {
    let subdir = format!("typst/packages/local/{}/{}", package.name, package.version);

    let dest = dirs::data_dir()
        .expect("failed to locate /local")
        .join(&subdir);

    if dest.exists() {
        println!("{}:{} already exists - skipping", package.name, package.version);
        return Ok(());
    }

    fs::create_dir_all(&dest).context("failed to create typst package bundle /local")?;

    let options = fs_extra::dir::CopyOptions {
        skip_exist: true,
        content_only: true,
        ..Default::default()
    };

    copy(&package.path, &dest, &options)?;
    println!("installed {}:{}", package.name, package.version);

    Ok(())
}
