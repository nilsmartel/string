mod util;

use itertools::join;
use std::io::Write;
use structopt::StructOpt;

const STDOUT_WRITE_ERROR: &'static str = "failed to write to stdout";

#[derive(StructOpt, Debug)]
#[structopt(about = "Cli for common string operations. Takes input from stdin.")]
enum StringCommand {
    /// Extract a part of a given string.
    Substr {
        #[structopt(default_value = "0", short, long)]
        start: usize,
        #[structopt(short, long)]
        end: usize,
    },
    /// Split up a string by a separator and print the parts on separate lines
    Split {
        #[structopt(default_value = " ", short)]
        separator: String,
    },
    /// Returns the length of string
    Length,
    /// Replace all matching words
    Replace {
        #[structopt(short, long = "match")]
        matching: String,
        #[structopt(short, long)]
        with: String,
    },
    /// Pick a single line by linenumber
    Line {
        #[structopt(short)]
        /// starting at 0
        number: usize,
    },
}

fn main() {
    let config: StringCommand = StringCommand::from_args();
    let input = util::stdin_as_string();

    use StringCommand::*;
    match config {
        Substr { start, end } => {
            println!("{}", substr(&input, start, end));
        }
        Split { separator } => {
            split(&input, &separator);
        }
        Length => println!("{}", input.len()),
        Replace { matching, with } => {
            let result = join(input.split(&matching), &with);
            println!("{}", result);
        }
        Line { number } => println!("{}", pick_line(&input, number)),
    }
}

fn pick_line(input: &str, number: usize) -> &str {
    if let Some((_, line)) = input
        .split("\n")
        .enumerate()
        .find(|(index, _)| *index == number)
    {
        line
    } else {
        eprintln!("input does not have enough lines");
        std::process::exit(1);
    }
}

fn split(input: &str, separator: &str) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    for line in input.split(separator) {
        lock.write(line.as_bytes()).expect(STDOUT_WRITE_ERROR);
        lock.write(b"\n").expect(STDOUT_WRITE_ERROR);
    }
}

fn substr(input: &str, start: usize, end: usize) -> String {
    if start > end {
        eprintln!("start value must be smaller than end value");
        std::process::exit(1);
    }

    let amount = end - start;

    input.chars().skip(start).take(amount).collect()
}
