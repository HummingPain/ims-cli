use std::fs::read_to_string;
use std::io::stdout;

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use log::{info, warn};
use ims_cli;
use clap::Parser;

fn main() -> Result<()> {
    info!("starting up");

    let pb = ProgressBar::new(100);
    warn!("oops, nothing implemented!");
    for i in 0..2 {
        let args = ims_cli::Cli::parse();

        let content = read_to_string(&args.path)
            .with_context(|| format!("could not read file `{}`", args.path.display()))?;

        ims_cli::find_matches(&args, content, &mut stdout());

        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}
