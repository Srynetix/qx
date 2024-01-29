use color_eyre::Result;
use std::path::{Path, PathBuf};

use qx_core::{Configuration, Context, Environment, Resolvable};
use tracing::{debug, info};

use crate::{
    io::FileAccess,
    models::{ConfigurationModel, EnvironmentModel},
};

pub struct ConfigurationStorage<'a, F: FileAccess> {
    pub file_access: &'a F,
}

impl<'a, F: FileAccess> ConfigurationStorage<'a, F> {
    pub fn new(file_access: &'a F) -> Self {
        Self { file_access }
    }

    pub fn get_default_configuration_path(&self) -> PathBuf {
        self.file_access
            .user_data_directory()
            .join("qx")
            .join("config.yaml")
    }

    pub fn read_from_path(&self, path: &Path) -> Result<Configuration> {
        info!(
            message = "Reading configuration",
            path = ?path
        );

        let data = self.file_access.read_to_string(path)?;
        let configuration: ConfigurationModel = serde_yaml::from_str(&data)?;
        let mut configuration = self.configuration_from_serde_model(configuration);

        let context = configuration.variables.clone();
        configuration.resolve(&context);

        debug!(
            message = "Configuration parsed and resolved",
            configuration = ?configuration
        );

        Ok(configuration)
    }

    pub fn write_to_path(&self, configuration: &Configuration, path: &Path) -> Result<()> {
        if let Some(parent_dir) = path.parent() {
            info!(
                message = "Creating folder",
                path = ?parent_dir
            );
            self.file_access.create_dir_all(parent_dir)?;
        }

        info!(
            message = "Writing configuration",
            path = ?path
        );

        self.file_access.write(
            path,
            serde_yaml::to_string(&self.configuration_to_serde_model(configuration))?,
        )?;

        Ok(())
    }

    fn configuration_to_serde_model(&self, configuration: &Configuration) -> ConfigurationModel {
        ConfigurationModel {
            version: env!("CARGO_PKG_VERSION").to_string(),
            system: Some(configuration.system.clone()),
            variables: Some(configuration.variables.0.clone()),
            environments: Some(
                configuration
                    .environments
                    .iter()
                    .map(|(k, e)| (k.clone(), self.environment_to_serde_model(e)))
                    .collect(),
            ),
        }
    }

    fn configuration_from_serde_model(&self, model: ConfigurationModel) -> Configuration {
        Configuration {
            system: model.system.unwrap_or_default(),
            variables: Context::new(model.variables.unwrap_or_default()),
            environments: model
                .environments
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k.clone(), self.environment_from_serde_model(k, v)))
                .collect(),
        }
    }

    fn environment_to_serde_model(&self, environment: &Environment) -> EnvironmentModel {
        EnvironmentModel {
            description: environment.description.clone(),
            actions: Some(environment.actions.to_vec()),
        }
    }

    fn environment_from_serde_model(&self, name: String, model: EnvironmentModel) -> Environment {
        Environment {
            name,
            description: model.description,
            actions: model.actions.unwrap_or_default(),
        }
    }
}
