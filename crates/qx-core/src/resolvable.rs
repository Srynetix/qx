use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::context::Context;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{(.*?)\}").unwrap());

pub trait Resolvable {
    fn resolve(&mut self, vars: &Context) -> Result<()>;
}

impl Resolvable for String {
    fn resolve(&mut self, vars: &Context) -> Result<()> {
        let replace_fn = |caps: &Captures| -> String {
            let variable_name = caps.get(1).unwrap().as_str();
            if let Some(value) = vars.get(variable_name) {
                value.clone()
            } else {
                panic!("Variable '{variable_name}' not found");
            }
        };

        let result = RE.replace_all(self, &replace_fn);
        *self = result.to_string();

        Ok(())
    }
}

impl Resolvable for PathBuf {
    fn resolve(&mut self, vars: &Context) -> Result<()> {
        let mut s = self.to_string_lossy().to_string();
        s.resolve(vars)?;

        *self = PathBuf::from(s);

        Ok(())
    }
}

impl<T: Resolvable> Resolvable for Option<T> {
    fn resolve(&mut self, vars: &Context) -> Result<()> {
        if let Some(value) = self.as_mut() {
            value.resolve(vars)?;
        }

        Ok(())
    }
}

impl<T: Resolvable> Resolvable for Vec<T> {
    fn resolve(&mut self, vars: &Context) -> Result<()> {
        for value in self.iter_mut() {
            value.resolve(vars)?;
        }

        Ok(())
    }
}
