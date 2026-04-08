use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "my-dev-env-restorer",
    about = "Sync dotfiles from this repo and open program download links."
)]
pub struct Cli {
    #[arg(long, help = "Copy repo-managed configs to this machine")]
    pub configs: bool,
    #[arg(long, help = "Pull local config changes back into this repo")]
    pub pull: bool,
    #[arg(long, help = "Open the program download links for this platform")]
    pub links: bool,
    #[arg(
        long,
        help = "Print planned work without changing files or opening links"
    )]
    pub dry_run: bool,
}

impl Cli {
    pub fn parse_and_validate() -> Result<Self> {
        let cli = Self::parse();

        if !cli.configs && !cli.pull && !cli.links {
            return Err(anyhow!(
                "Nothing to do. Use at least one of --configs, --pull, or --links."
            ));
        }

        if cli.configs && cli.pull {
            return Err(anyhow!(
                "--configs and --pull are opposites. Run them separately."
            ));
        }

        Ok(cli)
    }
}
