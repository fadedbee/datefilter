mod args;
mod filter;

use std::{collections::BTreeSet, fs::{self, DirEntry}, process::exit};

use anyhow::Result;
use args::Args;
use clap::Parser;
use log::{debug, error, info, warn};

/*
 * Run with "RUST_LOG="debug" cargo run".
 */
fn main() -> Result<()> {
    env_logger::init();
    debug!("datefilter starting...");

    /* Parse the command line arguments into an Args struct. */
    let args = Args::parse();
    debug!("args {args:?}");

    let entry_results = match fs::read_dir(args.path) {
        Ok(entry_results) => entry_results,
        Err(err) => {
            error!("Error: {err}");
            exit(-1);
        },
    };

    /* Remove error results. */
    let entries = entry_results.filter_map(|x| x.ok());

    /* Turn directory entries into file and directory names. */
    let names = entries.map(|x| x.file_name());

    /* Remove all the name which should not be deleted by the pipeline. */
    //let remaining = names.filter(date_filter);

    /* Write out the names which should be deleted by the pipeline. */
    //println!("{remaining:?}");

    Ok(())
}


