use crate::{
    actions::Action, context::Context, resolvable::Resolvable, ActionContext, CommandExecutor,
};
use color_eyre::Result;

#[derive(Debug)]
pub struct Environment {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
}

impl Environment {
    pub fn boot<E: CommandExecutor>(&self, context: &ActionContext<E>) -> Result<()> {
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
    fn resolve(&mut self, ctx: &Context) {
        self.description.resolve(ctx);
        for action in &mut self.actions {
            action.resolve(ctx);
        }
    }
}
