use std::{collections::HashMap, path::Path, process::Command};

#[derive(Debug, Clone, Default)]
pub struct System(pub HashMap<String, String>);

impl System {
    pub fn new(values: HashMap<String, String>) -> Self {
        Self(values)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    fn get_web_browser_target(&self) -> Option<&String> {
        self.0.get("web_browser_path")
    }

    fn get_web_browser_args(&self) -> Option<&String> {
        self.0.get("web_browser_args")
    }

    pub fn open_web_browser(&self, target: &str) -> color_eyre::Result<()> {
        if let Some(value) = self.get_web_browser_target() {
            Command::new(value)
                .args(self.get_web_browser_args())
                .arg(target)
                .spawn()?;
        } else {
            open::that(target)?;
        }

        Ok(())
    }

    fn get_editor_target(&self) -> Option<&String> {
        self.0.get("editor_path")
    }

    fn get_editor_args(&self) -> Option<&String> {
        self.0.get("editor_args")
    }

    pub fn open_editor(&self, target: &Path) -> color_eyre::Result<()> {
        if let Some(value) = self.get_editor_target() {
            Command::new(value)
                .args(self.get_editor_args())
                .arg(target)
                .status()?;
        } else {
            open::that(target)?;
        }

        Ok(())
    }

    pub fn open_file(&self, target: &Path) -> color_eyre::Result<()> {
        open::that(target)?;

        Ok(())
    }
}
