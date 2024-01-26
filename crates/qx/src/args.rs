use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use qx_core::Configuration;
use qx_storage::ConfigurationStorage;

pub enum ArgsCommand<'a> {
    Boot(Option<&'a String>),
    Interactive,
    Edit,
}

#[derive(Debug, Clone, Parser)]
#[clap(author)]
pub struct Args {
    /// Path to the configuration file to use
    #[arg(short, long)]
    pub configuration_path: Option<PathBuf>,

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
        if let Some(configuration_path) = &self.configuration_path {
            ConfigurationStorage::read_from_path(configuration_path)
        } else {
            let configuration_path = ConfigurationStorage::get_default_configuration_path();
            if !configuration_path.exists() {
                let configuration = Configuration::default();
                ConfigurationStorage::write_to_path(&configuration, &configuration_path)?;
                Ok(configuration)
            } else {
                ConfigurationStorage::read_from_path(&configuration_path)
            }
        }
    }

    pub fn get_configuration_path(&self) -> PathBuf {
        if let Some(configuration_path) = &self.configuration_path {
            configuration_path.clone()
        } else {
            ConfigurationStorage::get_default_configuration_path()
        }
    }

    pub fn command(&self) -> ArgsCommand {
        if self.edit {
            ArgsCommand::Edit
        } else if self.interactive {
            ArgsCommand::Interactive
        } else {
            ArgsCommand::Boot(self.environment.as_ref())
        }
    }
}
