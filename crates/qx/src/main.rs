pub mod args;

pub use args::Args;
use clap::{CommandFactory, Parser};
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use qx_core::{banner, ActionContext, Configuration, Environment};
use qx_tui::Choice;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn boot(config: &Configuration, env: &Environment) -> color_eyre::Result<()> {
    println!("  > Booting environment: {}", env.name);
    println!();

    let context = ActionContext {
        system: &config.system,
        context: &config.vars,
    };

    for action in &env.actions {
        action.execute(&context)?;
    }

    Ok(())
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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    show_banner();

    let args = Args::parse();
    setup_logging(args.verbose);

    let configuration = args.load_configuration_or_default()?;

    if args.interactive {
        let config_path = args.get_config_path();
        println!(
            "  > Opening configuration file {:?} in interactive mode",
            config_path
        );
        println!();

        let choice = qx_tui::run_loop(&configuration)?;
        match choice {
            Choice::Boot(env) => boot(&configuration, env)?,
            Choice::Quit | Choice::Continue => (),
        }

        return Ok(());
    }

    if args.edit {
        let config_path = args.get_config_path();
        println!(
            "  > Opening configuration file {:?} for edition",
            config_path
        );
        println!();

        configuration.system.open_editor(&config_path)?;
        return Ok(());
    }

    if let Some(env) = args.environment {
        let filtered_envs = configuration.find_environment(&env);
        if filtered_envs.is_empty() {
            eprintln!(
                "{}",
                format!("Error: no environment found with name '{}'", env).red()
            );
            std::process::exit(1);
        } else if filtered_envs.len() > 1 {
            let names = filtered_envs
                .iter()
                .map(|s| format!("{}", s))
                .join("\n  - ");
            eprintln!(
                "{}",
                format!("Error: multiple environments found:\n  - {}", names).red()
            );
            eprintln!();
            std::process::exit(1);
        } else {
            let env = &filtered_envs[0];
            boot(&configuration, env)?
        }
    } else {
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
    }

    Ok(())
}
