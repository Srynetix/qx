use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use url::Url;

use crate::{
    intent::{CommandCreationType, CommandIntent},
    resolvable::ResolvableClone,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct System {
    web_browser_path: Option<PathBuf>,
    web_browser_arguments: Option<Vec<String>>,
    editor_path: Option<PathBuf>,
    editor_arguments: Option<Vec<String>>,
    vscode_path: Option<PathBuf>,
    #[serde(default)]
    defaults_to_interactive: bool,
}

impl System {
    fn get_vscode_executable(&self) -> PathBuf {
        if let Some(value) = self.vscode_path.as_ref() {
            value.into()
        } else if cfg!(windows) {
            "%LOCALAPPDATA%\\Programs\\Microsoft VS Code\\Code.exe"
                .resolved_without_context()
                .into()
        } else {
            // Crossed fingers!
            "code".into()
        }
    }

    pub fn should_defaults_to_interactive(&self) -> bool {
        self.defaults_to_interactive
    }

    pub fn open_web_browser(&self, target: &Url) -> CommandIntent {
        if let Some(value) = self.web_browser_path.as_ref() {
            CommandIntent::Custom {
                target: value.into(),
                arguments: self
                    .web_browser_arguments
                    .iter()
                    .flatten()
                    .cloned()
                    .chain(std::iter::once(target.to_string()))
                    .collect(),
                working_directory: None,
                creation_type: CommandCreationType::Detach,
            }
        } else {
            CommandIntent::System {
                target: target.to_string().into(),
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
        if let Some(editor_path) = self.editor_path.as_ref() {
            CommandIntent::Custom {
                target: editor_path.into(),
                arguments: self
                    .editor_arguments
                    .iter()
                    .flatten()
                    .cloned()
                    .chain(std::iter::once(target.to_string_lossy().to_string()))
                    .collect(),
                working_directory: None,
                creation_type: CommandCreationType::Wait,
            }
        } else {
            CommandIntent::System {
                target: target.into(),
                creation_type: CommandCreationType::Wait,
            }
        }
    }

    pub fn open_file(&self, target: &Path) -> CommandIntent {
        CommandIntent::System {
            target: target.into(),
            creation_type: CommandCreationType::Detach,
        }
    }
}
