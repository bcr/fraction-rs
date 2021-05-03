use std::io::{self, Write};
use fraction::process_input;

fn output_prompt() {
    print!("? ");
    io::stdout().flush().unwrap();
}

fn read_input() -> Result<String, std::io::Error> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line = line.trim_end().to_string();
    Ok(line)
}

fn main() {
    loop {
        output_prompt();
        let line = read_input().ok().expect("Error reading user input");
        if line.is_empty() {
            break;
        }
        match process_input(&line) {
            Ok(result) => println!("= {}", result),
            Err(message) => println!("Input error: {}", message)
        }
    }
}
