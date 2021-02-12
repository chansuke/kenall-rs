use dotenv::dotenv;
use structopt::{clap, StructOpt};

use kenall_rs::run;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kenall-rs",
    about = "A command line tool for Kenall written in Rust"
)]
/// Show help with no argument
#[structopt(setting(clap::AppSettings::ArgRequiredElseHelp))]
struct Opt {
    /// Parse postal code as an argument
    #[structopt(help = "Enter the postal code that you want to search the address")]
    pub postal_code: String,
}

fn main() {
    dotenv().ok();

    let arg = Opt::from_args();
    let raw_code = arg.postal_code;

    run(&raw_code);
}
