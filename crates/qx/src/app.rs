use std::path::Path;

use clap::{CommandFactory, Parser};
use color_eyre::{owo_colors::OwoColorize, Result};
use itertools::Itertools;
use qx_core::{banner, ActionContext, Configuration, Environment};
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

pub struct App;

impl App {
    pub fn run() -> Result<AppStatusCode> {
        Self::setup_error_handling()?;
        Self::show_banner();

        let args = Args::parse();
        Self::setup_logging(args.verbose);

        let configuration = args.load_configuration_or_default()?;
        let configuration_path = args.get_configuration_path();

        match args.command() {
            ArgsCommand::Boot(filter) => Self::handle_environment(&configuration, filter),
            ArgsCommand::Edit => Self::handle_edit(&configuration, &configuration_path),
            ArgsCommand::Interactive => {
                Self::handle_interactive(&configuration, &configuration_path)
            }
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

    fn boot(configuration: &Configuration, environment: &Environment) -> Result<()> {
        println!("  > Booting environment: {}", environment.name);
        println!();

        let context = ActionContext {
            system: &configuration.system,
            context: &configuration.variables,
        };

        environment.boot(&context)
    }

    fn handle_environment(
        configuration: &Configuration,
        filter: Option<&String>,
    ) -> Result<AppStatusCode> {
        if let Some(filter) = filter {
            let filtered_environments = configuration.filter_environments(filter);
            if filtered_environments.is_empty() {
                Self::handle_environment_no_match(filter)
            } else if filtered_environments.len() > 1 {
                Self::handle_environment_too_many_matches(&filtered_environments, filter)
            } else {
                Self::handle_boot(configuration, filtered_environments[0])
            }
        } else {
            Self::handle_list_environments(configuration)
        }
    }

    fn handle_environment_no_match(filter: &str) -> Result<AppStatusCode> {
        eprintln!(
            "{}",
            format!("Error: no environment found with name '{}'", filter).red()
        );

        Ok(AppStatusCode::Error)
    }

    fn handle_environment_too_many_matches(
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

    fn handle_edit(
        configuration: &Configuration,
        configuration_path: &Path,
    ) -> Result<AppStatusCode> {
        println!(
            "  > Opening configuration file {:?} for edition",
            configuration_path
        );
        println!();

        configuration.system.open_editor(configuration_path)?;

        Ok(AppStatusCode::Success)
    }

    fn handle_interactive(
        configuration: &Configuration,
        configuration_path: &Path,
    ) -> Result<AppStatusCode> {
        println!(
            "  > Opening configuration file {:?} in interactive mode",
            configuration_path
        );
        println!();

        let choice = qx_tui::run_loop(configuration)?;
        match choice {
            Choice::Boot(env) => Self::boot(configuration, env)?,
            Choice::Quit | Choice::Continue => (),
        }

        Ok(AppStatusCode::Success)
    }

    fn handle_boot(
        configuration: &Configuration,
        environment: &Environment,
    ) -> Result<AppStatusCode> {
        Self::boot(configuration, environment)?;

        Ok(AppStatusCode::Success)
    }

    fn handle_list_environments(configuration: &Configuration) -> Result<AppStatusCode> {
        let envs = configuration
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
