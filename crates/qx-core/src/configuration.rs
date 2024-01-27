use std::collections::HashMap;

use itertools::Itertools;

use crate::{context::Context, environment::Environment, resolvable::Resolvable, system::System};

#[derive(Debug, Default)]
pub struct Configuration {
    pub system: System,
    pub variables: Context,
    pub environments: HashMap<String, Environment>,
}

impl Configuration {
    pub fn filter_environments(&self, name: &str) -> Vec<&Environment> {
        if let Some(env) = self.environments.get(name) {
            return vec![env];
        }

        self.environments
            .iter()
            .filter(|(key, _)| key.starts_with(name))
            .map(|(_, value)| value)
            .sorted_by_key(|v| v.name.clone())
            .collect()
    }

    pub fn list_environment_names(&self) -> Vec<&Environment> {
        self.environments
            .values()
            .sorted_by_key(|v| v.name.clone())
            .collect()
    }
}

impl Resolvable for Configuration {
    fn resolve(&mut self, ctx: &Context) {
        for env in &mut self.environments.values_mut() {
            env.resolve(ctx);
        }
    }
}
