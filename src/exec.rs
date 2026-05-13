use std::{io::Write, process::Stdio};

pub fn execute(command: &[String], stdin_text: Option<&str>) -> String {
    let command_name = &command[0];
    let mut command = std::process::Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("failed to spawn process {:?}", command));

    if let Some(stdin_text) = stdin_text {
        let stdin = command
            .stdin
            .as_mut()
            .expect("failed to open stdin of command");
        stdin
            .write_all(stdin_text.as_bytes())
            .expect("failed to pipe command into shell");
    }

    let output = command
        .wait_with_output()
        .expect("failed to aquire programm output");

    let status: std::process::ExitStatus = output.status;
    if !status.success() {
        eprintln!("error executing command `{}`.\nProcess terminated with exit code {}.\nProgram output:\n{}",
            command_name,
            status,
            String::from_utf8(output.stderr).unwrap()
        );
        std::process::exit(1);
    }

    String::from_utf8(output.stdout).expect("programm output was not valid utf-8")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exec1() {
        let input = "printf hello";
        let result = execute(&[String::from("sh")], Some(input));
        let expected = "hello";

        assert_eq!(expected, result);
    }

    #[test]
    fn exec2() {
        let result = execute(&[String::from("printf"), String::from("hello")], None);
        let expected = "hello";

        assert_eq!(expected, result);
    }
}
