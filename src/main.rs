mod util;

use itertools::join;
use std::io::Write;
use structopt::StructOpt;

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
    Replace {
        #[structopt(short, long = "match")]
        matching: String,
        #[structopt(short, long)]
        with: String,
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
    }
}

fn split(input: &str, separator: &str) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    for line in input.split(separator) {
        lock.write(line.as_bytes())
            .expect("failed to write to stdout");
        lock.write(b"\n").unwrap();
    }
}

fn substr(input: &str, start: usize, end: usize) -> String {
    if start > end {
        println!("start value must be smaller than end value");
        std::process::exit(1);
    }

    let amount = end - start;

    input.chars().skip(start).take(amount).collect()
}
