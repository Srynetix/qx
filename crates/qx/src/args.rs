use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use qx_core::Configuration;
use qx_storage::{ConfigurationStorage, FileAccess};

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
    pub fn load_configuration_or_default<F: FileAccess>(
        &self,
        storage: &ConfigurationStorage<F>,
    ) -> Result<Configuration> {
        if let Some(configuration_path) = &self.configuration_path {
            storage.read_from_path(configuration_path)
        } else {
            let configuration_path = storage.get_default_configuration_path();
            if !storage.file_access.file_exists(&configuration_path) {
                let configuration = Configuration::default();
                storage.write_to_path(&configuration, &configuration_path)?;
                Ok(configuration)
            } else {
                storage.read_from_path(&configuration_path)
            }
        }
    }

    pub fn get_configuration_path<F: FileAccess>(
        &self,
        storage: &ConfigurationStorage<F>,
    ) -> PathBuf {
        if let Some(configuration_path) = &self.configuration_path {
            configuration_path.clone()
        } else {
            storage.get_default_configuration_path()
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
