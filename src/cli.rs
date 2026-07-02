use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum CaseStyle {
    /// lowercase
    Lower,
    /// UPPERCASE
    Upper,
}

#[derive(Parser, Debug)]
#[command(about = "Cli for common string operations. Takes input from stdin.")]
pub enum StringCommand {
    /// Transform upper- or lowercase
    #[command(subcommand)]
    Case(CaseStyle),
    /// Reverse order of lines
    Reverse,
    /// Extract part of a given string.
    Substr {
        #[arg()]
        start: usize,
        #[arg()]
        end: usize,
    },
    /// Split up a string by a separator and print the parts on separate lines
    Split {
        #[arg(default_value = " ")]
        separator: String,
    },
    /// Join lines with a separator into a single string
    Join {
        #[arg(default_value = " ")]
        separator: String,
    },
    /// Returns the length the input string
    Length,
    /// Replace all matching characters
    Replace {
        #[arg()]
        matching: String,
        #[arg()]
        with: String,
    },
    /// Pick a single line by index
    Line {
        #[arg()]
        /// starting at 0
        number: usize,
    },
    /// Interleave input and only print every nth line
    Interleave {
        #[arg()]
        /// starting at 0
        n: usize,
    },
    /// Output the set of input strings without repetitions, in order
    Distinct {
        #[arg(short)]
        /// Distinct entire lines, instead of individual words
        lines: bool,
    },
    /// Trim whitespace on lines and ignore empty ones
    Trim,
    /// Prints all chars on separate lines
    Chars,
    /// Useful for templating, replace sections of input with the output of a shell command or script
    Template {
        #[arg(default_value = "{{", long = "begin")]
        /// Delimiter indicating beginning of command
        begin: String,

        #[arg(default_value = "}}", long = "end")]
        /// Delimiter indicating end of command
        end: String,

        #[arg(default_value = "sh", long)]
        /// in which shell the commands should be piped
        shell: Vec<String>,

        #[arg(long = "raw-output")]
        /// don't trim new lines and whitespace of the start and end of output
        raw_output: bool,
    },
    /// Map each line of input to a subcommand.
    Each {
        /// If set, input will be passed as stdin
        #[arg(short = 's', long = "stdin", default_value_t = false)]
        stdin: bool,
        /// Name of value to be replaced. Default is ""
        #[arg(short = 'v', long = "var", default_value = "{}")]
        var: String,
        /// Command to be executed. Pass "string each -- <commands...>" so you can pass flags to the command.
        command: Vec<String>,
    },
}
