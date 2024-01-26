use crate::{actions::Action, context::Context, resolvable::Resolvable, ActionContext};
use color_eyre::Result;

#[derive(Debug)]
pub struct Environment {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
}

impl Environment {
    pub fn boot(&self, context: &ActionContext) -> Result<()> {
        for action in &self.actions {
            action.execute(context)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.name, self.description))
    }
}

impl Resolvable for Environment {
    fn resolve(&mut self, ctx: &Context) -> color_eyre::Result<()> {
        for action in &mut self.actions {
            action.resolve(ctx)?;
        }

        Ok(())
    }
}
