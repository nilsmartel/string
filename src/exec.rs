use std::{io::Write, process::Stdio};

pub fn execute(text: &str, shell: &[&str]) -> String {
    let mut command = std::process::Command::new(shell[0])
        .args(&shell[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("failed to spawn process in shell {:?}", shell));

    {
        let stdin = command
            .stdin
            .as_mut()
            .expect("failed to open stdin of command");
        stdin
            .write_all(text.as_bytes())
            .expect("failed to pipe command into shell");
    }

    let output = command
        .wait_with_output()
        .expect("failed to aquire programm output");

    let status: std::process::ExitStatus = output.status;
    if !status.success() {
        eprintln!("error executing command `{}` in shell {}.\nProcess terminated with exit code {}.\nProgram output:\n{}", text, shell[0], status, String::from_utf8(output.stderr).unwrap());
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
        let result = execute(input, &["sh"]);
        let expected = "hello";

        assert_eq!(expected, result);
    }
}
