use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 28)]
    pub days: u32,

    #[arg(short, long, default_value_t = 12)]
    pub months: u32,

    #[arg(short, long, default_value_t = 100)]
    pub years: u32,
}
