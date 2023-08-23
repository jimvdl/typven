//! Several utility functions, such as `ls` and `clean`.

use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

use anyhow::{bail, Context};
use codespan_reporting::term::{self, termcolor::WriteColor};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table};
use walkdir::WalkDir;

use crate::{cli::CleanCommand, color_stream, package};

/// Lists the locally installed packages in table format.
///
/// If a package is not valid, i.e. does not contain a valid `typst.toml`, `ls`
/// will silently ignore that directory.
///
/// # Errors
///
/// No packages are installed.
pub fn ls() -> anyhow::Result<()> {
    let packages_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages/local");

    let packages = package::search(&packages_dir);

    if packages.is_empty() {
        bail!("no valid packages found");
    }

    let mut map: HashMap<String, String> = HashMap::new();
    for package in packages {
        map.entry(package.name)
            .and_modify(|e| {
                let mut v = package.version.to_string();
                v.insert(0, '\n');
                e.push_str(&v);
            })
            .or_insert(package.version.to_string());
    }

    let mut packages: Vec<(String, String)> = map.into_iter().collect();
    packages.sort();

    let mut table = Table::new();
    table
        .set_header(vec!["package", "versions"])
        .apply_modifier(UTF8_ROUND_CORNERS);

    packages.into_iter().for_each(|(k, v)| {
        table.add_row(vec![k, v]);
    });

    println!("{table}");

    Ok(())
}

/// Clean the local package directory.
///
/// There are a few possible ways a clean is performed (in order):
/// 1. If a valid package `name` is present clean all versions of that package.
/// 2. If a valid package `name` is present, as well as a valid package `version`,
/// clean that target version of that package.
/// 3. If there is no target package to clean, clean the /local directory of
/// every package leaving /local empty.
///
/// When cleaning the entire /local directory it will start searching for
/// packages to clean 1 level deep so invalid top-level packages that the
/// compiler would not recognize are not cleaned.
///
/// # Errors
///
/// A package with `name` and `version` does not exist.
/// A package with `name` does not exist.
/// The local package directory is empty.
pub fn clean(command: CleanCommand) -> anyhow::Result<()> {
    let root_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages/local");

    print_cleaning(&command).unwrap();

    if let Some(name) = command.name {
        if let Some(version) = command.version {
            let dir = root_dir.join(format!("{name}/{version}"));

            return fs::remove_dir_all(dir)
                .with_context(|| format!("failed to clean {name}:{version}, package not found"));
        }

        let dir = root_dir.join(&name);

        return fs::remove_dir_all(dir)
            .with_context(|| format!("failed to clean {name}, package bundle not found"));
    }

    let local_package_dirs: Vec<_> = WalkDir::new(&root_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .collect();

    if local_package_dirs.is_empty() {
        bail!("nothing to clean");
    }

    for dir in local_package_dirs {
        let _ = fs::remove_dir_all(dir.path());
    }

    Ok(())
}

/// Print that a clean is happening.
fn print_cleaning(command: &CleanCommand) -> io::Result<()> {
    let mut w = color_stream();
    let styles = term::Styles::default();

    w.set_color(&styles.header_help)?;
    write!(w, "cleaning")?;

    if let Some(name) = &command.name {
        if let Some(version) = &command.version {
            w.reset()?;
            return writeln!(w, " {name}:{version}");
        }

        w.reset()?;
        return writeln!(w, " {name}");
    }

    w.reset()?;
    writeln!(w, " all")
}
