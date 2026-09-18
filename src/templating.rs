use crate::exec::execute;

/// Finds commands within a template-string (e.g. sections delimited by `begin` and `end`,
/// excutes them sequentially and writes them back to the string.
pub fn execute_template(
    input: &str,
    shell: &[String],
    begin: &str,
    end: &str,
    trim: bool,
) -> anyhow::Result<String> {
    if shell.len() == 0 {
        anyhow::bail!("must specify a shell");
    }
    // 1 split text content and commands
    // 2 map commands to their execution output
    // 3 join and return

    // unwrapping is safe, this will always be Ok, rest will always be ""
    // TODO test this
    let ast = parse_template(input, begin, end);

    let mut buffer = String::with_capacity(256);

    for c in ast {
        buffer.push_str(c.text);
        if let Some(cmd) = c.command {
            let output = execute(shell, Some(cmd))?;
            let output = if trim { output.trim() } else { &output };
            buffer.push_str(output);
        }
    }

    Ok(buffer)
}

#[derive(PartialEq, Debug)]
struct TemplateBlock<'a> {
    text: &'a str,
    command: Option<&'a str>,
}

/// Parse a string and extract commands from within a given text
///
/// e.g.
/// ```
/// parse("hello {myCommand}! How are you?", "{", "}")
/// ```
/// will yield
/// ```
/// [
///     Content {
///         text: "hello ",
///         command: Some("myCommand"),
///     },
///     Content {
///         text: "! How are you?",
///         command: None,
///     }
/// ]
/// ```
fn parse_template<'a>(s: &'a str, begin: &str, end: &str) -> Vec<TemplateBlock<'a>> {
    // many1(|st| command(st, begin, end))(s)
    let mut rest = s;
    let mut result = Vec::new();
    loop {
        let Some((text, right)) = rest.split_once(begin) else {
            // No Commands left in the template, return the remaining string
            result.push(TemplateBlock {
                text: rest,
                command: None,
            });
            break;
        };

        let Some((command, rest1)) = right.split_once(end) else {
            // if a beginning is found without end, print error message and treat it as raw code.
            eprintln!("found unmatched delimiter: {begin}{right}\n expected '{end}' to be found");
            //
            // Treat this as if no command was in here, this is safer than executing arbitrary
            // data.
            result.push(TemplateBlock {
                text: rest,
                command: None,
            });
            break;
        };

        rest = rest1;
        let command = Some(command);
        result.push(TemplateBlock { text, command });
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn template1() {
        let input = "hello (echo world)";
        let result = execute_template(input, &["sh".to_string()], "(", ")", true).unwrap();
        let expected = "hello world";

        assert_eq!(expected, result);
    }

    #[test]
    fn template2() {
        let input = "Hey (echo VSauce), (echo Michael) here!";
        let result = execute_template(input, &["sh".to_string()], "(", ")", true).unwrap();
        let expected = "Hey VSauce, Michael here!";

        assert_eq!(expected, result);
    }

    #[test]
    fn template3() {
        let input = "Hey { echo VSauce }, { echo Michael } here!";
        let result = execute_template(input, &["sh".to_string()], "{", "}", true).unwrap();
        let expected = "Hey VSauce, Michael here!";

        assert_eq!(expected, result);
    }

    #[test]
    fn template4() {
        let input = "complex calculation: ^console.log(14)^";
        let result = execute_template(input, &["node".to_string()], "^", "^", true).unwrap();
        let expected = "complex calculation: 14";

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_empty() {
        let res = parse_template("", "{", "}");

        let content = TemplateBlock {
            text: "",
            command: None,
        };

        assert_eq!(res, vec![content]);
    }

    #[test]
    fn parse_single() {
        let res = parse_template("hello", "{", "}");

        let content = TemplateBlock {
            text: "hello",
            command: None,
        };

        assert_eq!(res, vec![content]);
    }

    #[test]
    fn parse_command() {
        let res = parse_template("hello {{echo USER}}!", "{{", "}}");

        let content = vec![
            TemplateBlock {
                text: "hello ",
                command: Some("echo USER"),
            },
            TemplateBlock {
                text: "!",
                command: None,
            },
        ];

        assert_eq!(res, content);
    }

    #[test]
    fn parse_multi() {
        let res = parse_template(
            "hello {{echo USER}}! How do you {{echo FEEL}} today?",
            "{{",
            "}}",
        );

        let content = vec![
            TemplateBlock {
                text: "hello ",
                command: Some("echo USER"),
            },
            TemplateBlock {
                text: "! How do you ",
                command: Some("echo FEEL"),
            },
            TemplateBlock {
                text: " today?",
                command: None,
            },
        ];

        assert_eq!(res, content);
    }
}
