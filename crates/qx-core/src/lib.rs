mod actions;
mod configuration;
mod context;
mod environment;
mod intent;
mod io;
mod resolvable;
mod system;

pub use actions::{Action, ActionContext};
pub use configuration::Configuration;
pub use context::Context;
pub use environment::Environment;
pub use io::{CommandExecutor, CommandExecutorIo};
pub use resolvable::Resolvable;
pub use system::System;

const PROJECT_URL: &str = "https://github.com/Srynetix/qx";

pub fn banner() -> String {
    format!(
        r"
    __    __  _  
  /'__`\ /\ \/'\ 
 /\ \L\ \\/>  </ 
 \ \___, \/\_/\_\
  \/___/\ \//\/_/
       \ \_\     
        \/_/   

   v. {}

   {}

   ",
        env!("CARGO_PKG_VERSION"),
        PROJECT_URL
    )
}
