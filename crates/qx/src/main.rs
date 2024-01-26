mod app;
mod args;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    let result = App::run()?;
    std::process::exit(result.as_code() as i32);
}
