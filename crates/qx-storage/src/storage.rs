use color_eyre::Result;
use std::path::{Path, PathBuf};

use qx_core::{Configuration, Context, Environment, Resolvable, System};
use tracing::{debug, info};

use crate::models::{ConfigurationModel, EnvironmentModel};

pub struct ConfigurationStorage;

impl ConfigurationStorage {
    pub fn get_default_configuration_path() -> PathBuf {
        let data_dir = dirs::data_dir().expect("Could not retrieve data directory.");
        data_dir.join("qx").join("config.yaml")
    }

    pub fn read_from_path(path: &Path) -> Result<Configuration> {
        info!(
            message = "Reading configuration",
            path = ?path
        );

        let data = std::fs::read_to_string(path)?;
        let configuration: ConfigurationModel = serde_yaml::from_str(&data)?;
        let mut configuration = Self::configuration_from_serde_model(configuration);

        let context = configuration.variables.clone();
        configuration.resolve(&context)?;

        debug!(
            message = "Configuration parsed and resolved",
            configuration = ?configuration
        );

        Ok(configuration)
    }

    pub fn write_to_path(configuration: &Configuration, path: &Path) -> Result<()> {
        if let Some(parent_dir) = path.parent() {
            info!(
                message = "Creating folder",
                path = ?parent_dir
            );
            std::fs::create_dir_all(parent_dir)?;
        }

        info!(
            message = "Writing configuration",
            path = ?path
        );

        std::fs::write(
            path,
            serde_yaml::to_string(&Self::configuration_to_serde_model(configuration))?,
        )?;

        Ok(())
    }

    fn configuration_to_serde_model(configuration: &Configuration) -> ConfigurationModel {
        ConfigurationModel {
            version: env!("CARGO_PKG_VERSION").to_string(),
            system: Some(configuration.system.0.clone()),
            variables: Some(configuration.variables.0.clone()),
            environments: Some(
                configuration
                    .environments
                    .iter()
                    .map(|(k, e)| (k.clone(), Self::environment_to_serde_model(e)))
                    .collect(),
            ),
        }
    }

    fn configuration_from_serde_model(model: ConfigurationModel) -> Configuration {
        Configuration {
            system: System::new(model.system.unwrap_or_default()),
            variables: Context::new(model.variables.unwrap_or_default()),
            environments: model
                .environments
                .unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k.clone(), Self::environment_from_serde_model(k, v)))
                .collect(),
        }
    }

    fn environment_to_serde_model(environment: &Environment) -> EnvironmentModel {
        EnvironmentModel {
            description: environment.description.clone(),
            actions: Some(environment.actions.to_vec()),
        }
    }

    fn environment_from_serde_model(name: String, model: EnvironmentModel) -> Environment {
        Environment {
            name,
            description: model.description,
            actions: model.actions.unwrap_or_default(),
        }
    }
}
