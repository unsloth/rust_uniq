use clap::{command, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    let mut file = open(&cli.input).map_err(|e| format!("{}: {}", cli.input, e))?;

    loop {
        let mut line = String::new();
        let bytes_in_line = file.read_line(&mut line)?;
        if bytes_in_line == 0 {
            break;
        }
        print!("{}", line);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
