use std::{fs, path::PathBuf};

use anyhow::{bail, Context};
use toml_edit::{Document, Item, Table};

#[derive(Debug)]
pub struct Manifest {
    pub data: toml_edit::Document,
    pub path: PathBuf,
}

impl Manifest {
    pub fn get_or_create() -> anyhow::Result<Self> {
        let data_dir = dirs::data_dir().expect("failed to locate data directory");

        let manifest_path = data_dir.join("typst/manifest.toml");
        let data: anyhow::Result<Document> = fs::read_to_string(&manifest_path)
            .and_then(|data| Ok(data.parse::<Document>().expect("")))
            .or_else(|_| {
                let mut doc = Document::new();
                doc.insert("packages", Item::Table(Table::new()));

                let data = doc.to_string();
                fs::write(&manifest_path, data)?;

                Ok(doc)
            });

        Ok(Self {
            data: data.expect("unable to create or get manifest.toml"),
            path: manifest_path,
        })
    }

    pub fn register(&mut self, package_name: &str, path: &PathBuf) -> anyhow::Result<()> {
        match self.data["packages"].as_table().unwrap().get(&package_name) {
            Some(_) => println!("the \"{}\" package is already registered", &package_name),
            None => {
                self.data["packages"].as_table_mut().unwrap().insert(
                    &package_name,
                    toml_edit::value(path.to_str().context("path failed to_str conversion")?),
                );

                self.write()?;

                return Ok(());
            }
        }

        bail!("failed to register \"{}\"", package_name)
    }

    pub fn get_package_path(&self, package_name: &str) -> anyhow::Result<PathBuf> {
        match self.data["packages"].as_table().unwrap().get(&package_name) {
            Some(path) => Ok(PathBuf::from(path.as_str().unwrap())),
            None => bail!(
                "package \"{}\" is not a registered package, run tm register <path>",
                package_name
            ),
        }
    }

    pub fn unregister(&mut self, package_name: &str) -> anyhow::Result<()> {
        self.data["packages"]
            .as_table_mut()
            .unwrap()
            .remove(&package_name);

        self.write()?;

        Ok(())
    }

    fn write(&self) -> std::io::Result<()> {
        fs::write(&self.path, self.data.to_string())
    }
}
