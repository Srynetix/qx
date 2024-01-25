use crate::{actions::Action, context::Context, resolvable::Resolvable};

#[derive(Debug)]
pub struct Environment {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.name, self.description))
    }
}

impl Resolvable for Environment {
    fn resolve(&mut self, vars: &Context) -> color_eyre::Result<()> {
        for action in &mut self.actions {
            action.resolve(vars)?;
        }

        Ok(())
    }
}
