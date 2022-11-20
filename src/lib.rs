use clap::{command, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

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
    let file = open(&cli.input).map_err(|e| format!("{}: {}", cli.input, e))?;

    let mut out_file: Box<dyn Write> = match &cli.output {
        Some(out) => Box::new(File::create(out)?),
        None => Box::new(io::stdout()),
    };

    let mut prev_line = String::new();
    let mut num_lines: usize = 0;

    let mut write_output = |num_lines: usize, prev_line: &str| -> MyResult<()> {
        if cli.count {
            write!(out_file, "{:>7} {}\n", num_lines, prev_line)?;
        } else {
            write!(out_file, "{}\n", prev_line)?;
        }
        Ok(())
    };

    for line in file.lines() {
        let line = line?;
        if line == prev_line || num_lines == 0 {
            num_lines += 1;
        } else {
            write_output(num_lines, &prev_line)?;
            num_lines = 1;
        }
        prev_line = line;
    }

    // inputting an empty file returns nothing,
    // num_lines should only equal 0 with an empty file
    if num_lines == 0 {
        return Ok(());
    }

    // Make sure to include last line
    write_output(num_lines, &prev_line)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
