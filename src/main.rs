mod exec;
mod templating;
mod util;
use templating::template;

use itertools::join;
use std::io::Write;
use structopt::StructOpt;

const STDOUT_WRITE_ERROR: &'static str = "failed to write to stdout";

#[derive(StructOpt, Debug)]
#[structopt(about = "Cli for common string operations. Takes input from stdin.")]
enum StringCommand {
    Reverse,
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
    /// Returns the length the input string
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
    /// Output the set of input strings without repetitions, in order
    Distinct {
        #[structopt(short)]
        /// Distinct entire lines, instead of individual words
        lines: bool,
    },
    /// Useful for templating, replace sections of input with the output of a shell command or script
    Template {
        #[structopt(default_value = "{{", long = "begin")]
        /// Delimiter indicating beginning of command
        begin: String,

        #[structopt(default_value = "}}", long = "end")]
        /// Delimiter indicating end of command
        end: String,

        #[structopt(default_value = "sh", long)]
        /// in which shell the commands should be piped
        shell: Vec<String>,

        #[structopt(long = "raw-output")]
        /// don't trim new lines and whitespace of the start and end of output
        raw_output: bool,
    },
}

fn main() {
    let command: StringCommand = StringCommand::from_args();
    let input = util::stdin_as_string();
    let mut output = std::io::stdout();

    perform_command(command, input, &mut output);
}

fn perform_command(command: StringCommand, input: String, output: &mut impl std::io::Write) -> std::io::Result<()> {
    use StringCommand::*;
    match command {
        Reverse => {
            for line in input.split("\n").collect::<Vec<_>>().iter().rev() {
                writeln!(output, "{}", line)?;
            }
        }
        Distinct { lines } => {
            let mut set = std::collections::BTreeSet::new();

            let separator = if lines {
                &['\n'][..]
            } else {
                &[' ', '\r', '\n', '\t'][..]
            };

            for line in input.split_terminator(separator) {
                if set.get(line).is_some() {
                    continue;
                }

                set.insert(line);
                writeln!(output, "{}", line)?;
            }
        }
        Substr { start, end } => {
            writeln!(output, "{}", substr(&input, start, end))?;
        }
        Split { separator } => {
            let result = join(input.split(&separator), "\n");
            write!(output, "{}", result)?;
        }
        Length => writeln!(output,"{}", input.len())?,
        Replace { matching, with } => {
            let result = join(input.split(&matching), &with);
            write!(output, "{}", result)?;
        }
        Line { number } => writeln!(output, "{}", pick_line(&input, number))?,
        Template {
            shell,
            begin,
            end,
            raw_output,
        } => {
            let shell: Vec<&str> = shell.iter().map(|s| s.as_str()).collect();

            let result = template(&input, &shell, &begin, &end, !raw_output);

            writeln!(output, "{}", result)?;
        }
    };

    Ok(())
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

fn substr(input: &str, start: usize, end: usize) -> String {
    if start > end {
        eprintln!("start value must be smaller than end value");
        std::process::exit(1);
    }

    let amount = end - start;

    input.chars().skip(start).take(amount).collect()
}
