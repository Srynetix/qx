use std::collections::HashMap;

use itertools::Itertools;

use crate::{context::Context, environment::Environment, resolvable::Resolvable, system::System};

#[derive(Debug, Default)]
pub struct Configuration {
    pub system: System,
    pub vars: Context,
    pub envs: HashMap<String, Environment>,
}

impl Configuration {
    pub fn find_environment(&self, name: &str) -> Vec<&Environment> {
        if let Some(env) = self.envs.get(name) {
            return vec![env];
        }

        self.envs
            .iter()
            .filter(|(key, _)| key.starts_with(name))
            .map(|(_, value)| value)
            .sorted_by_key(|v| v.name.clone())
            .collect()
    }

    pub fn list_environment_names(&self) -> Vec<&Environment> {
        self.envs
            .values()
            .sorted_by_key(|v| v.name.clone())
            .collect()
    }
}

impl Resolvable for Configuration {
    fn resolve(&mut self, vars: &Context) -> color_eyre::Result<()> {
        for env in &mut self.envs.values_mut() {
            env.resolve(vars)?;
        }

        Ok(())
    }
}
