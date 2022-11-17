use clap::{command, Parser};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Filter adjacent matching lines from INPUT, writing to OUTPUT"
)]
struct Cli {
    #[arg(short, long, help = "Prefix lines by the number of occurences")]
    count: bool,

    #[arg(default_value = "-", help = "Input file to read, or \"-\" for STDIN")]
    input: String,

    #[arg(help = "(OPTIONAL) Output file")]
    output: Option<String>,
}

pub fn run() -> MyResult<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    Ok(())
}
