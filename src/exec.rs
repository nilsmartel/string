use anyhow::{bail, Context};
use std::{io::Write, process::Stdio};

pub fn execute(command: &[String], stdin_text: Option<&str>) -> anyhow::Result<String> {
    let command_name = &command[0];
    let mut child = std::process::Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // captured, so the output of parallel commands can't smear over each other or the
        // progress bar. It is printed as part of the error message if the command fails.
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn process {:?}", command))?;

    let output = match stdin_text {
        None => child.wait_with_output(),
        // Feeding the command from a second thread, because writing the input and reading the
        // output have to happen at the same time. Filling up the input pipe while the command
        // is blocked on an output pipe nobody reads deadlocks both sides.
        Some(stdin_text) => {
            let mut stdin = child
                .stdin
                .take()
                .context("failed to open stdin of command")?;

            let (feeding, output) = std::thread::scope(|scope| {
                // dropping `stdin` when this thread ends closes the pipe, which is how the
                // command gets to see the end of its input
                let feeder = scope.spawn(move || stdin.write_all(stdin_text.as_bytes()));
                let output = child.wait_with_output();

                (feeder.join(), output)
            });

            match feeding {
                Err(_) => bail!("panic while piping input into command `{}`", command_name),
                // a command is free to stop reading early, like `head` does. That closes the
                // pipe under us, which is not an error of its own — the exit code decides.
                Ok(Err(e)) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
                Ok(result) => result.context("failed to pipe command into shell")?,
            }

            output
        }
    };

    let output = output.context("failed to aquire programm output")?;

    let status: std::process::ExitStatus = output.status;
    if !status.success() {
        bail!(
            "error executing command `{}`.\nProcess terminated with exit code {}.\nProgram output:\n{}",
            command_name,
            status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).context("programm output was not valid utf-8")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exec1() {
        let input = "printf hello";
        let result = execute(&[String::from("sh")], Some(input)).unwrap();
        let expected = "hello";

        assert_eq!(expected, result);
    }

    #[test]
    fn exec2() {
        let result = execute(&[String::from("printf"), String::from("hello")], None).unwrap();
        let expected = "hello";

        assert_eq!(expected, result);
    }

    #[test]
    fn exec_failure_reports_stderr() {
        let command = [
            String::from("sh"),
            String::from("-c"),
            String::from("echo boom >&2; exit 3"),
        ];

        let err = execute(&command, None).expect_err("command exits 3, so this must fail");
        let message = format!("{err:#}");

        assert!(
            message.contains("boom"),
            "stderr is missing from: {}",
            message
        );
        assert!(
            message.contains('3'),
            "exit code is missing from: {}",
            message
        );
    }

    /// Both pipes hold roughly 64kb, so echoing a megabyte back deadlocks any implementation
    /// that writes all of stdin before it starts reading stdout.
    #[test]
    fn large_input_does_not_deadlock() {
        let input = "abcdefgh\n".repeat(128 * 1024);
        let result = execute(&[String::from("cat")], Some(&input)).unwrap();

        assert_eq!(result.len(), input.len());
        assert_eq!(result, input);
    }

    /// a command that never reads its input leaves us writing into a closed pipe
    #[test]
    fn input_ignored_by_the_command_is_not_an_error() {
        let input = "abcdefgh\n".repeat(128 * 1024);
        let command = [
            String::from("sh"),
            String::from("-c"),
            String::from("echo done"),
        ];

        let result = execute(&command, Some(&input)).unwrap();

        assert_eq!(result, "done\n");
    }

    #[test]
    fn exec_failure_of_missing_binary() {
        let command = [String::from("definitely-not-an-existing-binary-42")];

        assert!(execute(&command, None).is_err());
    }
}
