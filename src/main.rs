mod args;
mod filter;

use std::io::{stdin, BufRead};

use anyhow::Result;
use args::Args;
use chrono::Local;
use clap::Parser;
use filter::{DateFilter, Outcome};
use log::debug;

/*
 * Run with "RUST_LOG="debug" cargo run".
 */
fn main() -> Result<()> {
    env_logger::init();
    debug!("datefilter starting...");

    /* Parse the command line arguments into an Args struct. */
    let args = Args::parse();
    debug!("args {args:?}");

    /* Make a DateFilter. */
    let today = Local::now().naive_local();
    let filter = DateFilter::new(today.into(), args.days, args.months, args.years);

    /* Read from stdin. */
    for line_result in stdin().lock().lines() {
        let Ok(line) = line_result else { break };
        for word in line.split_whitespace() {
            if filter.check(word) == Outcome::Pass {
                println!("{word}");
            }
        }
    }

    Ok(())
}
