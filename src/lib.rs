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
    let file = open(&cli.input).map_err(|e| format!("{}: {}", cli.input, e))?;

    let mut prev_line = String::new();
    let mut num_lines: usize = 0;
    let mut buf: Vec<String> = Vec::new();
    for file_line in file.lines() {
        if let Ok(line) = file_line {
            if line == prev_line || num_lines == 0 {
                num_lines += 1;
            } else {
                buf.push(format!(
                    "{}{}",
                    format_count(cli.count, num_lines),
                    prev_line.trim_end()
                ));
                num_lines = 1;
            }
            prev_line = line;
        }
    }
    buf.push(format!(
        "{}{}",
        format_count(cli.count, num_lines),
        prev_line.trim_end()
    ));

    for line in buf {
        println!("{}", line);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_count(count: bool, num_lines: usize) -> String {
    if count {
        format!("{:>7} ", num_lines)
    } else {
        "".to_string()
    }
}
