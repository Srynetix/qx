use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use qx_core::Action;

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct ConfigurationModel {
    pub version: String,
    pub system: Option<HashMap<String, String>>,
    pub variables: Option<HashMap<String, String>>,
    pub environments: Option<HashMap<String, EnvironmentModel>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct EnvironmentModel {
    pub description: String,
    pub actions: Option<Vec<Action>>,
}
