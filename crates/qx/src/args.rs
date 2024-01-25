use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use qx_core::Configuration;
use qx_storage::ConfigurationStorage;

#[derive(Debug, Clone, Parser)]
#[clap(author)]
pub struct Args {
    /// Config path to use
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,

    /// Edit mode
    #[arg(short, long)]
    pub edit: bool,

    /// Interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Environment match
    pub environment: Option<String>,
}

impl Args {
    pub fn load_configuration_or_default(&self) -> Result<Configuration> {
        if let Some(config_path) = &self.config_path {
            ConfigurationStorage::read_from_path(config_path)
        } else {
            let config_path = ConfigurationStorage::get_default_config_path();
            if !config_path.exists() {
                let config = Configuration::default();
                ConfigurationStorage::write_to_path(&config, &config_path)?;
                Ok(config)
            } else {
                ConfigurationStorage::read_from_path(&config_path)
            }
        }
    }

    pub fn get_config_path(&self) -> PathBuf {
        if let Some(config_path) = &self.config_path {
            config_path.clone()
        } else {
            ConfigurationStorage::get_default_config_path()
        }
    }
}
