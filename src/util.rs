use std::{collections::HashMap, fs};

use anyhow::Context;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table};
use semver::Version;

use crate::package::{is_package, Package};

pub fn ls() -> anyhow::Result<()> {
    let packages_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages/local");

    let packages: Vec<Package> = fs::read_dir(&packages_dir)?
        .filter_map(Result::ok)
        .filter_map(|e| e.path().is_dir().then(|| is_package(&e.path()))?)
        .collect();

    let mut map: HashMap<String, String> = HashMap::new();
    for package in packages {
        map.entry(package.name)
            .and_modify(|e| {
                let mut v = package.version.to_string();
                v.insert_str(0, "\n");
                e.push_str(&v);
            })
            .or_insert(package.version.to_string());
    }

    let mut table = Table::new();
    table
        .set_header(vec!["package", "versions"])
        .apply_modifier(UTF8_ROUND_CORNERS);

    map.into_iter().for_each(|(k, v)| {
        table.add_row(vec![k, v]);
    });

    println!("{table}");

    Ok(())
}

pub fn clean(name: Option<String>, version: Option<Version>) -> anyhow::Result<()> {
    let root_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages/local");

    if let Some(name) = name {
        if let Some(version) = version {
            let dir = root_dir.join(format!("{name}/{version}"));

            return fs::remove_dir_all(&dir)
                .with_context(|| format!("failed to clean {name}:{version}, package not found"));
        }

        let dir = root_dir.join(&name);

        return fs::remove_dir_all(&dir)
            .with_context(|| format!("failed to clean {name}, package bundle not found"));
    }

    fs::remove_dir_all(&root_dir).context("nothing to clean")
}
