mod cli;
mod exec;
mod pool;
mod progressbar;
mod templating;
mod util;

use templating::template;

use anyhow::bail;
use clap::Parser;
use itertools::join;

use crate::exec::execute;
use crate::progressbar::ProgressBar;

fn main() {
    let command: cli::StringCommand = cli::StringCommand::parse();
    // nothing gets piped into `completions`, so don't sit there waiting for stdin
    let input = match command {
        cli::StringCommand::Completions { .. } => String::new(),
        _ => util::stdin_as_string(),
    };
    let mut output = std::io::stdout();

    if let Err(e) = perform_command(command, input, &mut output) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
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

        fn text(&self) -> String {
            String::from_utf8_lossy(&self.buffer).into_owned()
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
    fn join_lines() {
        let cases = [
            ("hello\nworld", " ", "hello world\n"),
            ("a\nb\nc", ",", "a,b,c\n"),
            ("single", "-", "single\n"),
            ("hello\nworld\n", " ", "hello world\n"),
        ];
        for (input, sep, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                Join {
                    separator: sep.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_contains() {
        let cases = [
            ("hello world", "world", "hello world\n"),
            ("hello world", "hello", "hello world\n"),
            ("hello\nworld", "world", "world\n"),
            ("hello\nworld\n", "world", "world\n"),
            ("hello\nworld\n", "o", "hello\nworld\n"),
            ("hello\nworld", "worlds", ""),
            ("hello world", "helloj", ""),
        ];

        for (input, pattern, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                Contains {
                    not: false,
                    pattern: pattern.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_contains_not() {
        let cases = [
            ("hello world", "world", ""),
            ("hello\nworld", "world", "hello\n"),
            ("hello\nworld\n", "o", ""),
            ("hello\nworld", "worlds", "hello\nworld\n"),
        ];

        for (input, pattern, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                Contains {
                    not: true,
                    pattern: pattern.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_starts_with() {
        let cases = [
            ("hello world", "hello", "hello world\n"),
            ("hello\nworld\n", "hello", "hello\n"),
            ("hello\nworld", "w", "world\n"),
            // leading whitespace is ignored on both line and prefix
            ("    hello\nworld", "hello", "    hello\n"),
            ("hello\nworld", "  hello", "hello\n"),
            ("hello world", "ello", ""),
            ("ello\nworld\n", "hello", ""),
        ];

        for (input, prefix, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                StartsWith {
                    not: false,
                    prefix: prefix.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_starts_with_not() {
        let cases = [
            ("hello world", "hello", ""),
            ("hello\nworld", "hello", "world\n"),
            ("hello\nworld", "ello", "hello\nworld\n"),
        ];

        for (input, prefix, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                StartsWith {
                    not: true,
                    prefix: prefix.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_ends_with() {
        let cases = [
            ("hello world", "world", "hello world\n"),
            ("hello world\n", "world", "hello world\n"),
            ("hello\nworld", "world", "world\n"),
            // trailing whitespace is ignored on both line and suffix
            ("hello\nworld   ", "world", "world   \n"),
            ("hello\nworld", "world  ", "world\n"),
            ("hello world", "worl", ""),
            ("hello world\n", "worl", ""),
        ];

        for (input, suffix, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                EndsWith {
                    not: false,
                    suffix: suffix.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    #[test]
    fn filter_ends_with_not() {
        let cases = [
            ("hello world", "world", ""),
            ("hello\nworld", "world", "hello\n"),
            ("hello\nworld", "worl", "hello\nworld\n"),
        ];

        for (input, suffix, expected) in cases {
            let mut writer = TestWriter::new();
            perform_command(
                EndsWith {
                    not: true,
                    suffix: suffix.into(),
                },
                input.into(),
                &mut writer,
            )
            .unwrap();
            assert_eq!(writer, expected);
        }
    }

    fn each(threads: usize, sequential: bool, command: &[&str]) -> StringCommand {
        Each {
            stdin: false,
            var: "{}".into(),
            threads,
            sequential,
            command: command.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn each_sequential_keeps_input_order() {
        // the first line sleeps longest, so completion order is the reverse of the input
        let command = ["sh", "-c", "sleep 0.{}; echo {}"];

        for threads in [1, 4] {
            let mut writer = TestWriter::new();
            perform_command(
                each(threads, true, &command),
                "3\n2\n1\n".into(),
                &mut writer,
            )
            .unwrap();

            assert_eq!(writer, "3\n2\n1\n");
        }
    }

    #[test]
    fn each_unordered_returns_every_result() {
        let command = ["sh", "-c", "sleep 0.{}; echo {}"];

        let mut writer = TestWriter::new();
        perform_command(each(4, false, &command), "3\n2\n1\n".into(), &mut writer).unwrap();

        let text = writer.text();
        let mut lines: Vec<&str> = text.lines().collect();
        lines.sort();

        assert_eq!(lines, ["1", "2", "3"]);
    }

    #[test]
    fn each_reports_failures_but_finishes_the_rest() {
        // "two" is not a number, so `test` exits non-zero for that line only
        let command = ["sh", "-c", "test {} -gt 0 && echo {}"];

        let mut writer = TestWriter::new();
        let res = perform_command(each(1, true, &command), "1\ntwo\n3\n".into(), &mut writer);

        let err = res.expect_err("one command failed, so the run must fail");
        assert!(
            format!("{:#}", err).contains("1 of 3"),
            "unexpected error: {:#}",
            err
        );
        // the lines around the failure still made it through
        assert_eq!(writer, "1\n3\n");
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
) -> anyhow::Result<()> {
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
                writeln!(output, "{}", line)?;
            }
        }
        Interleave { n } => {
            for (i, line) in input.lines().enumerate() {
                if i % n == 0 {
                    writeln!(output, "{}", line)?;
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
            for line in input.split(&separator) {
                writeln!(output, "{}", line)?;
            }
        }
        Join { separator } => {
            let result = join(input.lines(), &separator);
            writeln!(output, "{}", result)?;
        }
        Contains { not, pattern } => {
            for line in input.lines() {
                if line.contains(&pattern) != not {
                    writeln!(output, "{line}")?;
                }
            }
        }
        StartsWith { not, prefix } => {
            let prefix = prefix.trim_start();
            for line in input.lines() {
                if line.trim_start().starts_with(&prefix) != not {
                    writeln!(output, "{line}")?;
                }
            }
        }
        EndsWith { not, suffix } => {
            let suffix = suffix.trim_end();
            for line in input.lines() {
                if line.trim_end().ends_with(&suffix) != not {
                    writeln!(output, "{line}")?;
                }
            }
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
            let result = template(&input, &shell, &begin, &end, !raw_output)?;
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
            threads,
            sequential,
            ref command,
        } => {
            let lines: Vec<&str> = input.lines().collect();
            let mut progress = ProgressBar::new(lines.len(), threads > 1);
            let mut failures = 0;

            pool::for_each(
                &lines,
                threads,
                sequential,
                // on the worker threads
                |line| {
                    let command: Vec<_> = command.iter().map(|s| s.replace(&var, line)).collect();
                    let input = if stdin { Some(*line) } else { None };

                    execute(&command, input)
                },
                // back on this thread, so nothing is ever written half way through
                |index, result| {
                    match result {
                        Ok(result) => {
                            if result.ends_with("\n") {
                                write!(output, "{}", result)?;
                            } else {
                                writeln!(output, "{}", result)?;
                            }
                        }
                        Err(e) => {
                            failures += 1;
                            eprintln!("line {}: {e:#}", index + 1);
                        }
                    }

                    // flush before redrawing, so the bar stays the last thing on the screen
                    output.flush()?;
                    progress.tick();
                    Ok(())
                },
            )?;

            if failures > 0 {
                bail!("{} of {} commands failed", failures, lines.len());
            }
        }
        Completions { shell } => cli::completions(shell, output),
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
