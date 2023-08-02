// mod manifest;
// mod cli;
// mod install;
// mod package;

// use clap::Parser;
// use manifest::Manifest;
// use cli::Cli;

// fn main() -> anyhow::Result<()> {
//     let cli = Cli::parse();
//     println!("{:?}", cli);

//     // TODO: might not even need a manifest because packages have their own
//     // version and have to remain available for backwards compatibility.
//     // let manifest = Manifest::new()?;

//     let res = match cli.command {
//         cli::Commands::Install{ raw_path } => install::package(raw_path),
//     };

//     if let Err(res) = res {
//         // failed
//     }

//     Ok(())
// }

use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use fs_extra::dir::copy;
use serde::{Deserialize, Serialize};
use toml_edit::{Document, Item, Table};

#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Register {
        path: PathBuf,
    },
    Install {
        name: String,
        version: Option<semver::Version>,
    },
    Clean {
        name: Option<String>,
        version: Option<semver::Version>,
    },
}

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

#[derive(Debug)]
pub struct Manifest {
    pub data: toml_edit::Document,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    let data_dir = dirs::data_dir().expect("failed to locate data directory");
    let install_dir = data_dir.join("typst/packages");

    fs::create_dir_all(&install_dir)
        .context("creating typst package bundle directory in /local")?;

    let manifest_path = data_dir.join("typst/manifest.toml");
    if !manifest_path.exists() {
        let mut doc = Document::new();
        doc.insert("packages", Item::Table(Table::new()));

        fs::write(&manifest_path, doc.to_string())?;
    }

    match cli.command {
        Commands::Register { path } => {
            let raw = fs::read_to_string(&manifest_path)?;
            let mut manifest = raw
                .parse::<Document>()
                .expect("failed to load manifest.toml");

            let package_name = path
                .file_name()
                .context("failed to grab package bundle name from directory")?
                .to_string_lossy();

            match manifest["packages"].as_table().unwrap().get(&package_name) {
                Some(_) => println!("the \"{}\" package is already registered", &package_name),
                None => {
                    manifest["packages"].as_table_mut().unwrap().insert(
                        &package_name,
                        toml_edit::value(path.to_str().context("path failed to_str conversion")?),
                    );
                    fs::write(&manifest_path, manifest.to_string())?;
                }
            }
        }
        Commands::Install { name, version } => {
            let raw = fs::read_to_string(&manifest_path)?;
            let manifest = raw
                .parse::<Document>()
                .expect("failed to load manifest.toml");

            let root_dir = match manifest["packages"].as_table().unwrap().get(&name) {
                Some(package_path) => PathBuf::from(package_path.as_str().unwrap()),
                None => bail!(
                    "package \"{}\" is not a registered package, run tm register <path>",
                    name
                ),
            };

            if let Some(version) = version {
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
                let manifest: PackageManifest =
                    toml::from_str(&raw).context("manifest corrupted")?;

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

                return Ok(());
            }

            // when no version was specified install/update all package entries
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
                let manifest: PackageManifest =
                    toml::from_str(&raw).context("manifest corrupted")?;

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
        }
        Commands::Clean { name, version } => {
            if let Some(name) = name {
                let raw = fs::read_to_string(&manifest_path)?;
                let mut manifest = raw
                    .parse::<Document>()
                    .expect("failed to load manifest.toml");

                let root_dir = dirs::data_dir()
                    .expect("failed to locate data directory")
                    .join("typst/packages");

                if let Some(version) = version {
                    fs::remove_dir_all(root_dir.join(format!("{}-{}", name, version)))
                        .context(format!("cleaning \"{}\" package version {}", name, version))?;

                    return Ok(())
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
                
                manifest["packages"].as_table_mut().unwrap().remove(&name);
                fs::write(&manifest_path, manifest.to_string())?;

                return Ok(())
            }

            let typst_dir = dirs::data_dir()
                .expect("failed to locate data directory")
                .join("typst");

            fs::remove_dir_all(typst_dir).context("cleaning all packages")?;
        }
    }

    Ok(())
}
