mod app;
mod args;

use app::App;
use color_eyre::Result;
use qx_core::CommandExecutorIo;
use qx_storage::FileAccessIo;

fn main() -> Result<()> {
    let executor_io = CommandExecutorIo::default();
    let file_access_io = FileAccessIo::default();

    let result = App::run(&executor_io, &file_access_io)?;
    std::process::exit(result.as_code() as i32);
}
