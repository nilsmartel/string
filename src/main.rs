use std::io::Read;
use structopt::StructOpt;
fn main() {
    let config: StringCommand = StringCommand::from_args();

    let input = {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    use StringCommand::*;
    match config {
        Substr { start, end } => {
            // please don't do this at home
            println!("{}", &input[start..end]);
        }
        Split { pattern } => {
            for line in input.split(&pattern) {
                // don't do this, it's unperformant and people will know you're bad at rust
                println!("{}", line);
            }
        }
    }
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
    /// Split up a string by a pattern and print the parts on separate lines
    Split {
        #[structopt(default_value = " ", short)]
        pattern: String,
    },
}
