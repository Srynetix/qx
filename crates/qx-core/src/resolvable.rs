use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::context::Context;

static VARIABLE_INTERPOLATION_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{(.*?)\}").unwrap());
static WINDOWS_ENVIRON_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r"%(.*?)%").unwrap());

pub trait Resolvable {
    fn resolve(&mut self, ctx: &Context) -> Result<()>;
}

pub trait ResolvableClone {
    type Output;

    fn resolved(&self, ctx: &Context) -> Result<Self::Output>;
}

impl Resolvable for String {
    fn resolve(&mut self, ctx: &Context) -> Result<()> {
        let replace_fn = |caps: &Captures| -> String {
            let variable_name = caps.get(1).unwrap().as_str();
            if let Some(value) = ctx.get(variable_name) {
                value.clone()
            } else if let Ok(value) = std::env::var(variable_name) {
                value.clone()
            } else {
                panic!("Variable '{variable_name}' not found");
            }
        };

        let result = VARIABLE_INTERPOLATION_RGX.replace_all(self, &replace_fn);
        let result = WINDOWS_ENVIRON_RGX.replace_all(&result, &replace_fn);
        *self = result.to_string();

        Ok(())
    }
}

impl Resolvable for PathBuf {
    fn resolve(&mut self, ctx: &Context) -> Result<()> {
        let mut s = self.to_string_lossy().to_string();
        s.resolve(ctx)?;

        *self = PathBuf::from(s);

        Ok(())
    }
}

impl<T: Resolvable> Resolvable for Option<T> {
    fn resolve(&mut self, ctx: &Context) -> Result<()> {
        if let Some(value) = self.as_mut() {
            value.resolve(ctx)?;
        }

        Ok(())
    }
}

impl<T: Resolvable> Resolvable for Vec<T> {
    fn resolve(&mut self, ctx: &Context) -> Result<()> {
        for value in self.iter_mut() {
            value.resolve(ctx)?;
        }

        Ok(())
    }
}

impl<T: Resolvable + Clone> ResolvableClone for T {
    type Output = T;

    fn resolved(&self, ctx: &Context) -> Result<Self::Output> {
        let mut cloned_value = self.clone();
        cloned_value.resolve(ctx)?;

        Ok(cloned_value)
    }
}

impl<'a> ResolvableClone for &'a str {
    type Output = String;

    fn resolved(&self, ctx: &Context) -> Result<Self::Output> {
        let mut value = self.to_string();
        value.resolve(ctx)?;

        Ok(value)
    }
}
