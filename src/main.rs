mod exec;
mod templating;
mod util;

use templating::template;

use itertools::join;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum CaseStyle {
    /// lowercase
    Lower,
    /// UPPERCASE
    Upper,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Cli for common string operations. Takes input from stdin.")]
enum StringCommand {
    /// Transform upper- or lowercase
    Case(CaseStyle),
    /// Reverse order of lines
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
    /// Trim whitespace on lines and ignore empty ones
    Trim,
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

fn main() -> std::io::Result<()> {
    let command: StringCommand = StringCommand::from_args();
    let input = util::stdin_as_string();
    let mut output = std::io::stdout();

    perform_command(command, input, &mut output)
}

#[cfg(test)]
mod tests {
    use super::{perform_command, StringCommand, StringCommand::*};
    use std::fmt::Formatter;

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl std::fmt::Debug for TestWriter {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let s = String::from_utf8_lossy(&self.buffer);

            s.fmt(f)
        }
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buffer: Vec::with_capacity(128),
            }
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl PartialEq<&str> for TestWriter {
        fn eq(&self, other: &&str) -> bool {
            self.buffer == other.as_bytes()
        }
    }

    #[test]
    fn reverse() {
        let cases = [
            ("öüä", "öüä\n"),
            ("öüä\n", "öüä\n"),
            ("hello\nworld", "world\nhello\n"),
            ("hello\n\nworld", "world\nhello\n"),
            ("hello\nworld\n", "world\nhello\n"),
            ("hello\n\nworld\n", "world\nhello\n"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(Reverse, input.into(), &mut writer).unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn distinct_words() {
        let cases = [
            ("hello hello hello", "hello\n"),
            ("hello world", "hello\nworld\n"),
            ("1 2 3 4", "1\n2\n3\n4\n"),
            ("öüä öüä äüö äää ööö üüü", "öüä\näüö\näää\nööö\nüüü\n"),
            ("öüä äüö äää ööö üüü", "öüä\näüö\näää\nööö\nüüü\n"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(Distinct { lines: false }, input.into(), &mut writer).unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn distinct_lines() {
        let cases = [
            ("hello\nhello\nhello", "hello\n"),
            ("hello hello\nhello", "hello hello\nhello\n"),
            ("hello\nworld", "hello\nworld\n"),
            ("1 2 3 4", "1 2 3 4\n"),
            ("öüä\nöüä\näüö\näää\nööö\nüüü", "öüä\näüö\näää\nööö\nüüü\n"),
            ("öüä\näüö\näää\nööö\nüüü", "öüä\näüö\näää\nööö\nüüü\n"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(Distinct { lines: true }, input.into(), &mut writer).unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn substring() {
        let cases = [
            ("abcd", "cd\n"),
            ("abc", "c\n"),
            ("abcdefg", "cd\n"),
            ("äbcdefg", "cd\n"),
            ("öüä", "ä\n"),
            ("öüäß", "äß\n"),
            ("öüäß€", "äß\n"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(Substr { start: 2, end: 4 }, input.into(), &mut writer).unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn lowercase() {
        let cases = [
            ("abcdefg", "abcdefg"),
            ("ABCDEFG", "abcdefg"),
            ("AbcdEFG", "abcdefg"),
            ("AbcdEFGöüäÖÜÄ", "abcdefgöüäöüä"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                StringCommand::Case(super::CaseStyle::Lower),
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn uppercase() {
        let cases = [
            ("abcdefg", "ABCDEFG"),
            ("ABCDEFG", "ABCDEFG"),
            ("AbcdEFG", "ABCDEFG"),
            ("AbcdEFGöüäÖÜÄ", "ABCDEFGÖÜÄÖÜÄ"),
        ];

        for (input, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                StringCommand::Case(super::CaseStyle::Upper),
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn trim() {
        let input = "
        Hello

            World\t
        ";
        let expected = "Hello\nWorld\n";

        let mut writer = TestWriter::new();
        perform_command(
            Trim,
            input.into(),
            &mut writer,
        )
        .unwrap();
        assert_eq!(writer, expected);
    }
}

fn perform_command(
    command: StringCommand,
    input: String,
    output: &mut impl std::io::Write,
) -> std::io::Result<()> {
    use StringCommand::*;
    match command {
        Case(c) => match c {
            CaseStyle::Lower => {
                let input = input.to_lowercase();
                write!(output, "{}", input)?;
            }
            CaseStyle::Upper => {
                let input = input.to_uppercase();
                write!(output, "{}", input)?;
            }
        },
        Reverse => {
            for line in input
                .split('\n')
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .filter(|l| !l.is_empty())
            {
                writeln!(output, "{}", line)?;
            }
        }
        Trim => {
            for line in input
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
            {
                writeln!(output, "{}", line)?
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
        Length => writeln!(output, "{}", input.len())?,
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
        .split('\n')
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
