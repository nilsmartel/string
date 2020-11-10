use std::io::Read;

pub fn stdin_as_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read stdin to string.");
    buffer
}
