use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use color_eyre::{owo_colors::OwoColorize, Result};
use itertools::Itertools;
use qx_core::{banner, ActionContext, CommandExecutor, Configuration, Environment};
use qx_storage::{ConfigurationStorage, FileAccess};
use qx_tui::Choice;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::args::{Args, ArgsCommand};

pub enum AppStatusCode {
    Success,
    Error,
}

impl AppStatusCode {
    pub fn as_code(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Error => 1,
        }
    }
}

pub struct App<'a, E: CommandExecutor> {
    executor: &'a E,
    configuration: Configuration,
    configuration_path: PathBuf,
}

impl<'a, E: CommandExecutor> App<'a, E> {
    pub fn run<F: FileAccess>(executor: &'a E, file_access: &'a F) -> Result<AppStatusCode> {
        Self::setup_error_handling()?;
        Self::show_banner();

        let args = Args::parse();
        Self::setup_logging(args.verbose);

        let storage = ConfigurationStorage::new(file_access);
        let configuration = args.load_configuration_or_default(&storage)?;
        let configuration_path = args.get_configuration_path(&storage);

        let app = Self {
            configuration,
            configuration_path,
            executor,
        };

        match args.command() {
            ArgsCommand::Boot(filter) => app.handle_environment(filter),
            ArgsCommand::Edit => app.handle_edit(),
            ArgsCommand::Interactive => app.handle_interactive(),
        }
    }

    fn setup_error_handling() -> Result<()> {
        color_eyre::install()
    }

    fn setup_logging(verbose: bool) {
        let level_to_use = if verbose {
            LevelFilter::DEBUG
        } else {
            LevelFilter::ERROR
        };

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(
                EnvFilter::builder()
                    .with_default_directive(level_to_use.into())
                    .from_env_lossy(),
            )
            .init();
    }

    fn show_banner() {
        println!("{}", banner());
    }

    fn boot(&self, environment: &Environment) -> Result<()> {
        println!("  > Booting environment: {}", environment.name);
        println!();

        let context = ActionContext {
            system: &self.configuration.system,
            context: &self.configuration.variables,
            executor: self.executor,
        };

        environment.boot(&context)
    }

    fn handle_environment(&self, filter: Option<&String>) -> Result<AppStatusCode> {
        if let Some(filter) = filter {
            let filtered_environments = self.configuration.filter_environments(filter);
            if filtered_environments.is_empty() {
                self.handle_environment_no_match(filter)
            } else if filtered_environments.len() > 1 {
                self.handle_environment_too_many_matches(&filtered_environments, filter)
            } else {
                self.handle_boot(filtered_environments[0])
            }
        } else {
            self.handle_list_environments()
        }
    }

    fn handle_environment_no_match(&self, filter: &str) -> Result<AppStatusCode> {
        eprintln!(
            "{}",
            format!("Error: no environment found with name '{}'", filter).red()
        );

        Ok(AppStatusCode::Error)
    }

    fn handle_environment_too_many_matches(
        &self,
        environments: &[&Environment],
        filter: &str,
    ) -> Result<AppStatusCode> {
        let names = environments.iter().map(|s| format!("{}", s)).join("\n  - ");

        eprintln!(
            "{}",
            format!(
                "Error: multiple environments found for filter '{}':\n  - {}",
                filter, names
            )
            .red()
        );
        eprintln!();

        Ok(AppStatusCode::Error)
    }

    fn handle_edit(&self) -> Result<AppStatusCode> {
        println!(
            "  > Opening configuration file {:?} for edition",
            self.configuration_path
        );
        println!();

        let intent = self
            .configuration
            .system
            .open_editor(&self.configuration_path);
        self.executor.execute(intent)?;

        Ok(AppStatusCode::Success)
    }

    fn handle_interactive(&self) -> Result<AppStatusCode> {
        println!(
            "  > Opening configuration file {:?} in interactive mode",
            self.configuration_path
        );
        println!();

        let choice = qx_tui::run_loop(&self.configuration)?;
        match choice {
            Choice::Boot(env) => self.boot(env)?,
            Choice::Quit | Choice::Continue => (),
        }

        Ok(AppStatusCode::Success)
    }

    fn handle_boot(&self, environment: &Environment) -> Result<AppStatusCode> {
        self.boot(environment)?;

        Ok(AppStatusCode::Success)
    }

    fn handle_list_environments(&self) -> Result<AppStatusCode> {
        let envs = self
            .configuration
            .list_environment_names()
            .iter()
            .map(|e| format!("{}", e))
            .join("\n  - ");

        let mut cmd = <Args as CommandFactory>::command();
        cmd.print_help().unwrap();

        println!();
        println!("{}", "Available environments:".underline().bold());
        println!("  - {}", envs);
        println!();

        Ok(AppStatusCode::Error)
    }
}
