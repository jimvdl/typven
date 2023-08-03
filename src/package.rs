use std::{collections::HashMap, fs, path::PathBuf};

use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table};
use semver::Version;
use serde::{Deserialize, Serialize};

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

pub fn ls() -> anyhow::Result<()> {
    let packages_dir = dirs::data_dir()
        .expect("failed to locate data directory")
        .join("typst/packages");

    let packages: Vec<(String, Version)> = fs::read_dir(&packages_dir)?
        .filter_map(Result::ok)
        .filter_map(|e| {
            let raw = fs::read_to_string(e.path().join("typst.toml")).ok()?;
            let manifest: PackageManifest = toml::from_str(&raw).ok()?;

            Some((manifest.package.name, manifest.package.version))
        })
        .collect();

    let mut map: HashMap<String, String> = HashMap::new();
    for (name, version) in packages {
        map.entry(name)
            .and_modify(|e| {
                let mut v = version.to_string();
                v.insert_str(0, "\n");
                e.push_str(&v);
            })
            .or_insert(version.to_string());
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
