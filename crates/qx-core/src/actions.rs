use std::{path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{context::Context, resolvable::Resolvable, system::System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRun {
    pub target: String,
    pub args: Option<Vec<String>>,
    pub working_directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionShowMessage {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOpenUrl {
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOpenFile {
    pub target: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionVSCode {
    pub target: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Run(ActionRun),
    OpenFile(ActionOpenFile),
    ShowMessage(ActionShowMessage),
    OpenUrl(ActionOpenUrl),
    #[serde(rename = "vscode")]
    VSCode(ActionVSCode),
}

pub struct ActionContext<'a> {
    pub system: &'a System,
    pub context: &'a Context,
}

impl Action {
    pub fn execute(&self, ctx: &ActionContext) -> color_eyre::Result<()> {
        match self {
            Action::Run(action) => {
                info!(
                    message = "Running Run action",
                    target = ?action.target,
                    args = ?action.args,
                    working_directory = ?action.working_directory
                );

                let mut command = Command::new(&action.target);
                if let Some(args) = &action.args {
                    command.args(args);
                }

                if let Some(working_directory) = &action.working_directory {
                    command.current_dir(working_directory);
                }

                command.spawn()?;
            }
            Action::OpenFile(action) => {
                info!(
                    message = "Running OpenFile action",
                    target = ?action.target
                );

                ctx.system.open_file(&action.target)?;
            }
            Action::ShowMessage(action) => {
                info!(
                    message = "Running ShowMessage action",
                    data = ?action.message,
                );

                println!("{}", action.message);
            }
            Action::OpenUrl(action) => {
                info!(
                    message = "Running OpenUrl action",
                    target = ?action.target
                );

                ctx.system.open_web_browser(&action.target)?;
            }
            Action::VSCode(action) => {
                info!(
                    message = "Running VSCode action",
                    target = ?action.target
                );

                ctx.system.open_vscode(&action.target)?
            }
        }

        Ok(())
    }

    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::Run(action) => {
                format!(
                    "Run application {:?} with args {:?} and working directory {:?}",
                    action.target, action.args, action.working_directory
                )
            }
            Self::OpenFile(action) => {
                format!("Open file or folder {:?}", action.target)
            }
            Self::OpenUrl(action) => {
                format!("Open URL {:?}", action.target)
            }
            Self::ShowMessage(action) => {
                format!("Show message {:?}", action.message)
            }
            Self::VSCode(action) => {
                format!("Open VSCode on target {:?}", action.target)
            }
        }
    }
}

impl Resolvable for Action {
    fn resolve(&mut self, ctx: &Context) -> color_eyre::Result<()> {
        match self {
            Self::Run(cmd) => {
                cmd.target.resolve(ctx)?;
                cmd.args.resolve(ctx)?;
                cmd.working_directory.resolve(ctx)?;
            }
            Self::ShowMessage(cmd) => {
                cmd.message.resolve(ctx)?;
            }
            Self::OpenUrl(cmd) => {
                cmd.target.resolve(ctx)?;
            }
            Self::OpenFile(cmd) => {
                cmd.target.resolve(ctx)?;
            }
            Self::VSCode(cmd) => {
                cmd.target.resolve(ctx)?;
            }
        }

        Ok(())
    }
}
