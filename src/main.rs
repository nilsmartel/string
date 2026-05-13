mod cli;
mod exec;
mod templating;
mod util;

use templating::template;

use clap::Parser;
use itertools::join;

use crate::exec::execute;

fn main() -> std::io::Result<()> {
    let command: cli::StringCommand = cli::StringCommand::parse();
    let input = util::stdin_as_string();
    let mut output = std::io::stdout();

    perform_command(command, input, &mut output)
}

#[cfg(test)]
mod tests {
    use super::{cli::StringCommand, cli::StringCommand::*, perform_command};
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
                StringCommand::Case(super::cli::CaseStyle::Lower),
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
                StringCommand::Case(super::cli::CaseStyle::Upper),
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
        perform_command(Trim, input.into(), &mut writer).unwrap();
        assert_eq!(writer, expected);
    }
}

fn perform_command(
    command: cli::StringCommand,
    input: String,
    output: &mut impl std::io::Write,
) -> std::io::Result<()> {
    use cli::CaseStyle;
    use cli::StringCommand::*;
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
        Interleave { n } => {
            for (i, line) in input.lines().enumerate() {
                if i % n == 0 {
                    writeln!(output, "{}", line)?
                }
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
            let result = template(&input, &shell, &begin, &end, !raw_output);

            writeln!(output, "{}", result)?;
        }
        Chars => {
            for c in input.chars() {
                writeln!(output, "{}", c)?;
            }
        }
        Each {
            stdin,
            var,
            ref command,
        } => {
            for line in input.lines() {
                let command: Vec<_> = command.iter().map(|s| s.replace(&var, line)).collect();
                let input = if stdin { Some(line) } else { None };

                let result = execute(&command, input);
                writeln!(output, "{}", result)?;
            }
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
