mod bump_version;

use clap::Parser;
use color_eyre::Result;

use crate::bump_version::BumpVersion;

#[derive(Parser, Debug)]
enum Args {
    /// Bump package version.
    BumpVersion { version: String },
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    match args {
        Args::BumpVersion { version } => {
            BumpVersion::new(version).run()?;
        }
    }

    Ok(())
}
