mod cli;
mod manifest;
mod repo;
mod sync;

use anyhow::Result;

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse_and_validate()?;
    sync::run(cli)
}
