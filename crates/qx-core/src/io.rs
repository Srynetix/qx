use std::process::{Command, Stdio};

use color_eyre::Result;

use crate::intent::{CommandCreationType, CommandIntent};

pub trait CommandExecutor {
    fn execute(&self, intent: CommandIntent) -> Result<()>;
}

#[derive(Default)]
pub struct CommandExecutorIo {}

impl CommandExecutor for CommandExecutorIo {
    fn execute(&self, intent: CommandIntent) -> Result<()> {
        match intent {
            CommandIntent::Custom {
                target,
                arguments,
                working_directory,
                creation_type,
            } => {
                let mut command = Command::new(target);
                command.args(&arguments);

                if let Some(dir) = working_directory {
                    command.current_dir(dir);
                }

                match creation_type {
                    CommandCreationType::Detach => {
                        command
                            .stdout(Stdio::null())
                            .stdin(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()?;
                    }
                    CommandCreationType::DetachWithOutput => {
                        command.spawn()?;
                    }
                    CommandCreationType::Wait => {
                        command.status()?;
                    }
                }
            }
            CommandIntent::System {
                target,
                creation_type,
            } => match creation_type {
                CommandCreationType::Detach => {
                    open::that_detached(target)?;
                }
                CommandCreationType::DetachWithOutput | CommandCreationType::Wait => {
                    open::that(target)?;
                }
            },
        }

        Ok(())
    }
}
