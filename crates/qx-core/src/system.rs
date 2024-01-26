use std::{collections::HashMap, path::Path, process::Command};

use color_eyre::Result;

use crate::{resolvable::ResolvableClone, Context};

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

    pub fn open_web_browser(&self, target: &str) -> Result<()> {
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

    fn get_vscode_executable(&self) -> Result<String> {
        if let Some(value) = self.0.get("vscode_path") {
            Ok(value.clone())
        } else if cfg!(windows) {
            "%LOCALAPPDATA%\\Programs\\Microsoft VS Code\\Code.exe".resolved(&Context::empty())
        } else {
            // Crossed fingers!
            Ok("code".into())
        }
    }

    pub fn open_vscode(&self, target: &Path) -> Result<()> {
        Command::new(self.get_vscode_executable()?)
            .arg(target)
            .spawn()?;

        Ok(())
    }

    pub fn open_editor(&self, target: &Path) -> Result<()> {
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

    pub fn open_file(&self, target: &Path) -> Result<()> {
        open::that(target)?;

        Ok(())
    }
}
