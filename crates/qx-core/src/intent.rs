use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandCreationType {
    Wait,
    Detach,
    DetachWithOutput,
}

impl CommandCreationType {
    pub fn detach() -> Self {
        Self::Detach
    }
}

#[derive(Debug, Clone)]
pub enum CommandIntent {
    System {
        target: PathBuf,
        creation_type: CommandCreationType,
    },
    Custom {
        target: PathBuf,
        arguments: Vec<String>,
        working_directory: Option<PathBuf>,
        creation_type: CommandCreationType,
    },
}
