use std::{collections::HashMap, path::Path};

use crate::{
    intent::{CommandCreationType, CommandIntent},
    resolvable::ResolvableClone,
};

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

    fn get_editor_target(&self) -> Option<&String> {
        self.0.get("editor_path")
    }

    fn get_editor_args(&self) -> Option<&String> {
        self.0.get("editor_args")
    }

    fn get_vscode_executable(&self) -> String {
        if let Some(value) = self.0.get("vscode_path") {
            value.clone()
        } else if cfg!(windows) {
            "%LOCALAPPDATA%\\Programs\\Microsoft VS Code\\Code.exe".resolved_without_context()
        } else {
            // Crossed fingers!
            "code".into()
        }
    }

    pub fn open_web_browser(&self, target: &str) -> CommandIntent {
        if let Some(value) = self.get_web_browser_target() {
            CommandIntent::Custom {
                target: value.into(),
                arguments: self
                    .get_web_browser_args()
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .chain(std::iter::once(target.into()))
                    .collect(),
                working_directory: None,
                creation_type: CommandCreationType::Detach,
            }
        } else {
            CommandIntent::System {
                target: target.into(),
                creation_type: CommandCreationType::Detach,
            }
        }
    }

    pub fn open_vscode(&self, target: &Path) -> CommandIntent {
        let mut arguments = vec![];
        if target.starts_with("vscode-remote://") {
            arguments.push("--folder-uri".into());
        }

        arguments.push(target.to_string_lossy().to_string());

        CommandIntent::Custom {
            target: self.get_vscode_executable(),
            arguments,
            working_directory: None,
            creation_type: CommandCreationType::Detach,
        }
    }

    pub fn open_editor(&self, target: &Path) -> CommandIntent {
        if let Some(target) = self.get_editor_target() {
            CommandIntent::Custom {
                target: target.clone(),
                arguments: self
                    .get_editor_args()
                    .into_iter()
                    .map(|a| a.to_owned())
                    .chain(std::iter::once(target.into()))
                    .collect(),
                working_directory: None,
                creation_type: CommandCreationType::Wait,
            }
        } else {
            CommandIntent::System {
                target: target.to_string_lossy().to_string(),
                creation_type: CommandCreationType::Wait,
            }
        }
    }

    pub fn open_file(&self, target: &Path) -> CommandIntent {
        CommandIntent::System {
            target: target.to_string_lossy().to_string(),
            creation_type: CommandCreationType::Detach,
        }
    }
}
