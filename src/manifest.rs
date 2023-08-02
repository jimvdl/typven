use std::{fs, path::PathBuf};
use toml_edit::{Document, Item, Table};

use crate::package::Package;

#[derive(Default)]
pub struct Manifest {
    pub data: toml_edit::Document,
}

impl Manifest {
    pub fn new() -> anyhow::Result<Self> {
        let path = PathBuf::from("typst/packages/manifest.toml");

        let dir = dirs::cache_dir().expect("cache_dir not found").join(&path);
        if !dir.exists() {
            let mut doc = Document::new();
            doc.insert("packages", Item::Table(Table::new()));

            fs::write(dir, doc.to_string())?;

            Ok(Self { data: doc })
        } else {
            let data = fs::read_to_string(dir)?;

            Ok(Self {
                data: data.parse::<Document>().expect("invalid manifest format"),
            })
        }
    }

    pub fn add(&mut self, package: &Package, path: &PathBuf) {
        let pkg_index = format!("{}:{}", package.name, package.version);

        if self.data["packages"].as_table().unwrap().contains_key(&pkg_index) {
            // package already added
            return;
        }

        self.data["packages"]
            .as_table_mut()
            .unwrap()
            .insert(&pkg_index, toml_edit::value(path.to_str().unwrap()));

        let path = PathBuf::from("typst/packages/manifest.toml");

        if let Some(cache_dir) = dirs::cache_dir() {
            let dir = cache_dir.join(&path);

            fs::write(dir, self.data.to_string()).unwrap();
        }
    }
}
