#[allow(unused_parens)]

/*
 * Technical Debt:
 *
 * This "args" module exists only to supress incorrect warnings about parentheses.
 * https://stackoverflow.com/questions/75235520/how-to-stop-warnings-about-parentheses-required-by-clap-macros
 *
 * Once this is fixed, the args module and all the "pub(super)"s can be removed.
 */

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 28)]
    pub days: u16,

    #[arg(short, long, default_value_t = 12)]
    pub months: u8,

    #[arg(short, long, default_value_t = 100)]
    pub years: u16,
}