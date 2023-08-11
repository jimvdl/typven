use std::fs;

use anyhow::Context;
use semver::Version;

pub fn all(name: Option<String>, version: Option<Version>) -> anyhow::Result<()> {
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
