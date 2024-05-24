use std::fmt::Write;
use std::path::PathBuf;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::{
    context::Context,
    intent::{CommandCreationType, CommandIntent},
    resolvable::Resolvable,
    CommandExecutor, System,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRun {
    pub target: PathBuf,
    pub args: Option<Vec<String>>,
    pub working_directory: Option<PathBuf>,
    #[serde(default = "CommandCreationType::detach")]
    pub creation_type: CommandCreationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionShowMessage {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOpenUrl {
    pub target: Url,
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

pub struct ActionContext<'a, E: CommandExecutor> {
    pub system: &'a System,
    pub context: &'a Context,
    pub executor: &'a E,
}

impl Action {
    pub fn execute<E: CommandExecutor>(&self, ctx: &ActionContext<E>) -> color_eyre::Result<()> {
        match self {
            Action::Run(action) => {
                info!(
                    message = "Running Run action",
                    target = ?action.target,
                    args = ?action.args,
                    working_directory = ?action.working_directory
                );

                let intent = CommandIntent::Custom {
                    target: action.target.clone(),
                    arguments: action
                        .args
                        .as_ref()
                        .map(|value| value.iter().map(Into::into).collect())
                        .unwrap_or_default(),
                    working_directory: action.working_directory.clone(),
                    creation_type: action.creation_type.clone(),
                };

                ctx.executor.execute(intent)?;
            }
            Action::OpenFile(action) => {
                info!(
                    message = "Running OpenFile action",
                    target = ?action.target
                );

                let intent = ctx.system.open_file(&action.target);
                ctx.executor.execute(intent)?;
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
                    target = %action.target
                );

                let intent = ctx.system.open_web_browser(&action.target);
                ctx.executor.execute(intent)?;
            }
            Action::VSCode(action) => {
                info!(
                    message = "Running VSCode action",
                    target = ?action.target
                );

                let intent = ctx.system.open_vscode(&action.target);
                ctx.executor.execute(intent)?;
            }
        }

        Ok(())
    }

    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::Run(action) => {
                let mut output = String::new();

                write!(output, "Run application {:?}", action.target).unwrap();
                if let Some(args) = &action.args {
                    write!(
                        output,
                        " with args [{}]",
                        args.iter()
                            .map(|a| format!("\"{a}\""))
                            .collect_vec()
                            .join(", ")
                    )
                    .unwrap();
                }

                if let Some(cwd) = &action.working_directory {
                    write!(output, " with working directory {:?}", cwd).unwrap();
                }

                writeln!(
                    output,
                    " using creation type \"{:?}\"",
                    action.creation_type
                )
                .unwrap();

                output
            }
            Self::OpenFile(action) => {
                format!("Open file or folder {:?}", action.target)
            }
            Self::OpenUrl(action) => {
                format!("Open URL \"{}\"", action.target)
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
    fn resolve(&mut self, ctx: &Context) {
        match self {
            Self::Run(cmd) => {
                cmd.target.resolve(ctx);
                cmd.args.resolve(ctx);
                cmd.working_directory.resolve(ctx);
            }
            Self::ShowMessage(cmd) => {
                cmd.message.resolve(ctx);
            }
            Self::OpenUrl(cmd) => {
                cmd.target.resolve(ctx);
            }
            Self::OpenFile(cmd) => {
                cmd.target.resolve(ctx);
            }
            Self::VSCode(cmd) => {
                cmd.target.resolve(ctx);
            }
        }
    }
}
