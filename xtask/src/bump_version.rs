use color_eyre::Result;
use std::path::PathBuf;
use toml_edit::Document;

pub struct BumpVersion {
    version: String,
}

impl BumpVersion {
    pub fn new(version: String) -> Self {
        Self { version }
    }

    pub fn run(&self) -> Result<()> {
        let packages = self.scan_packages()?;
        for package in packages {
            self.set_package_version(package, &self.version)?;
        }

        Ok(())
    }

    fn scan_packages(&self) -> Result<Vec<PathBuf>> {
        let xtask_pkg = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let crates_dir = xtask_pkg.parent().unwrap().join("crates");

        Ok(std::fs::read_dir(crates_dir)?
            .filter_map(|path| {
                if let Ok(path) = path.map(|p| p.path().join("Cargo.toml")) {
                    if path.exists() {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect())
    }

    fn set_package_version(&self, package_file: PathBuf, version: &str) -> Result<()> {
        let file_contents = std::fs::read_to_string(&package_file)?;
        let mut contents = file_contents.parse::<Document>()?;
        contents["package"]["version"] = toml_edit::value(version);

        std::fs::write(package_file, contents.to_string())?;

        Ok(())
    }
}
