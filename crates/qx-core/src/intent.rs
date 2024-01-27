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
        target: String,
        creation_type: CommandCreationType,
    },
    Custom {
        target: String,
        arguments: Vec<String>,
        working_directory: Option<PathBuf>,
        creation_type: CommandCreationType,
    },
}
