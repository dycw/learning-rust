fn main() {
    let command_line: std::env::Args = std::env::args();
    for argument in command_line {
        print!("[{}]", argument);
    }
}
