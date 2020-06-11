use itertools::join;
use std::io::Read;
use structopt::StructOpt;
fn main() {
    let config: StringCommand = StringCommand::from_args();
    let input = stdin_as_string();

    use StringCommand::*;
    match config {
        Substr { start, end } => {
            // please don't do this at home. Byte indexing isn't okay.
            println!("{}", &input[start..end]);
        }
        Split { separator } => {
            for line in input.split(&separator) {
                // performs poorly, locks stdout over and over again
                println!("{}", line);
            }
        }
        Length => println!("{}", input.len()),
        Replace { matching, with } => {
            let result = join(input.split(&matching), &with);
            println!("{}", result);
        }
    }
}

fn stdin_as_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read stdin to string.");
    buffer
}

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
