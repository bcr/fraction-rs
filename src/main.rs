use std::io::{self, Write};

mod fraction;
use fraction::Fraction;

fn process_input(input: &str) -> Result<String, String> {
    let mut elements = input.split_whitespace();
    let f1: Fraction = elements.next().unwrap().parse().unwrap();
    let operator = elements.next().unwrap();
    let f2: Fraction = elements.next().unwrap().parse().unwrap();

    let mut result = match operator {
        "*" => Fraction { numerator: f1.numerator * f2.numerator, denominator: f1.denominator * f2.denominator },
        "/" => Fraction { numerator: f1.numerator * f2.denominator, denominator: f1.denominator * f2.numerator },
        "+" => Fraction { numerator: (f1.numerator * f2.denominator) + (f2.numerator * f1.denominator), denominator: f1.denominator * f2.denominator},
        "-" => Fraction { numerator: (f1.numerator * f2.denominator) - (f2.numerator * f1.denominator), denominator: f1.denominator * f2.denominator},
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
}
