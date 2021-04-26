use std::io::{self, Write};

mod fraction;
use fraction::Fraction;

fn process_input(input: &str) -> Result<String, String> {
    let mut elements = input.split_whitespace();
    let f1 = elements.next().unwrap().parse::<Fraction>().unwrap();
    let operator = elements.next().unwrap();
    let f2 = elements.next().unwrap().parse::<Fraction>().unwrap();

    let mut result = match operator {
        "*" => f1 * f2,
        "/" => f1 / f2,
        "+" => f1 + f2,
        "-" => f1 - f2,
        _ => return Err(String::from("Unknown operator"))
    };

    result = Fraction::reduce(result)?;
    Ok(format!("{}", result))
}

fn output_prompt() {
    print!("? ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    loop {
        output_prompt();
        let line = read_input();
        match process_input(&line) {
            Ok(result) => println!("= {}", result),
            Err(message) => println!("Input error: {}", message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply() {
        assert_eq!("10", process_input("2 * 5").unwrap());
        assert_eq!("1/8", process_input("1/4 * 1/2").unwrap());
    }

    #[test]
    fn divide() {
        assert_eq!("2/5", process_input("2 / 5").unwrap());
        assert_eq!("1/2", process_input("1/4 / 1/2").unwrap());
    }

    #[test]
    fn add() {
        assert_eq!("3/4", process_input("1/2 + 1/4").unwrap());
    }

    #[test]
    fn sub() {
        assert_eq!("1/3", process_input("1/2 - 1/6").unwrap());
    }

    #[test]
    fn examples() {
        assert_eq!("1_7/8", process_input("1/2 * 3_3/4").unwrap());
        assert_eq!("3_1/2", process_input("2_3/8 + 9/8").unwrap());
    }

    #[test]
    fn spaces() {
        // Operands and operators shall be separated by one or more spaces.
        assert_eq!("1_7/8", process_input("1/2      *    3_3/4").unwrap());
        assert_eq!("3_1/2", process_input("2_3/8  +  9/8").unwrap());
    }
}
