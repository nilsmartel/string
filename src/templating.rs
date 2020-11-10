// use itertools::join;
use std::{io::Write, process::Stdio};

use nom::{
    bytes::complete::{tag, take_until},
    combinator::opt,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn template(input: &str, shell: &[&str], begin: &str, end: &str) -> String {
    if shell.len() == 0 {
        eprintln!("must specify a shell");
        std::process::exit(1);
    }
    // 1 split text content and commands
    // 2 map commands to their execution output
    // 3 join and return

    // unwrapping is safe, this will always be Ok, rest will always be ""
    // TODO test this
    let ast = parse(input, begin, end).unwrap().1;

    let mut buffer = String::with_capacity(256);

    for c in ast {
        buffer.push_str(c.text);
        if let Some(cmd) = c.command {
            let output = execute(cmd, shell);
            buffer.push_str(&output);
        }
    }

    buffer
}

fn execute(script: &str, shell: &[&str]) -> String {
    let mut command = std::process::Command::new(shell[0])
        .args(&shell[1..])
        .stdin(Stdio::piped())
        .spawn()
        .expect(&format!("failed to spawn process in shell {:?}", shell));
    {
        let stdin = command
            .stdin
            .as_mut()
            .expect("failed to open stdin of command");
        stdin
            .write_all(script.as_bytes())
            .expect("failed to pipe command into shell");
    }

    let output = command
        .wait_with_output()
        .expect("failed to aquire programm output");

    let status: std::process::ExitStatus = output.status;
    if !status.success() {
        eprintln!("error executing command `{}` in shell {}.\nProcess terminated with exit code {}.\nProgram output:\n{}", script, shell[0], status, String::from_utf8(output.stderr).unwrap());
        std::process::exit(1);
    }

    String::from_utf8(output.stdout).expect("programm output was not valid utf-8")
}

#[derive(PartialEq, Debug)]
struct Content<'a> {
    text: &'a str,
    command: Option<&'a str>,
}

fn parse<'a>(s: &'a str, begin: &str, end: &str) -> IResult<&'a str, Vec<Content<'a>>> {
    many1(|s| command(s, begin, end))(s)
}

/// Parse the first command from a given text, returnes the rest of the text
///
/// e.g.
/// ```
/// parse("hello {myCommand}! How are you?", "{", "}")
/// ```
/// will yield
/// ```
/// Ok((
///     "! How are you?",
///     Content {
///         text: "hello ",
///         command: Some("myCommand"),
///     }
/// ))
/// ```
fn command<'a>(s: &'a str, begin: &str, end: &str) -> IResult<&'a str, Content<'a>> {
    let (rest, text) = opt(take_until(begin))(s)?;
    if text.is_none() {
        return Ok((
            "",
            Content {
                text: s,
                command: None,
            },
        ));
    }
    let text = text.unwrap();

    let (rest, command) = opt(preceded(tag(begin), terminated(take_until(end), tag(end))))(rest)?;

    Ok((rest, Content { text, command }))
}

#[cfg(test)]
mod test {
    use super::{command, parse, Content};
    #[test]
    fn parsing() {
        let res = command("hello {myCommand}! How are you?", "{", "}");
        assert!(res.is_ok());
        let res = res.unwrap();

        let rest = "! How are you?";
        let content = Content {
            text: "hello ",
            command: Some("myCommand"),
        };

        assert_eq!(res.0, rest);
        assert_eq!(res.1, content);
    }

    #[test]
    fn parsing_no_command() {
        let res = command("hello user! How are you?", "{", "}");
        assert!(res.is_ok());
        let res = res.unwrap();

        let rest = "";
        let content = Content {
            text: "hello user! How are you?",
            command: None,
        };

        assert_eq!(res.0, rest);
        assert_eq!(res.1, content);
    }
}
