use std::fmt;
use std::io::{self, Write};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numerator = self.numerator.abs();
        let denominator = self.denominator;
        let is_negative = self.numerator < 0;

        if is_negative {
            write!(f, "-").unwrap();
        }

        if denominator == 1 {
            let whole = numerator / denominator;
            write!(f, "{}", whole)
        }
        else {
            let mut remaining_numerator = numerator;
            if numerator > denominator {
                let whole = numerator / denominator;
                remaining_numerator -= whole * denominator;
                write!(f, "{}_", whole).unwrap();
            }
            write!(f, "{}/{}", remaining_numerator, denominator)
        }
    }
}

impl FromStr for Fraction {
    type Err = ParseIntError;

    fn from_str(fraction_string: &str) -> Result<Self, Self::Err> {
        let has_whole = fraction_string.contains("_");
        let has_fraction = fraction_string.contains("/");
        let mut whole: i32 = 0;
        let mut numerator: i32 = 0;
        let mut denominator: i32 = 1;
        let mut remaining_string = fraction_string;
    
        if !has_whole && !has_fraction {
            whole = remaining_string.parse::<i32>().unwrap();
        }
    
        if has_whole {
            let mut elements = remaining_string.split("_");
            whole = elements.next().unwrap().parse::<i32>().unwrap();
            remaining_string = elements.next().unwrap();
        }
    
        if has_fraction {
            let mut elements = remaining_string.split("/");
            numerator = elements.next().unwrap().parse::<i32>().unwrap();
            denominator = elements.next().unwrap().parse::<i32>().unwrap();
        }
    
        if whole < 0 {
            numerator = -numerator;
        }
    
        Ok(Fraction { numerator: (whole * denominator) + numerator, denominator: denominator })
    }
}

// https://stackoverflow.com/questions/18541832/c-sharp-find-the-greatest-common-divisor
fn gcd(a : i32, b : i32) -> i32 {
    if b == 0 {
        return a
    } else {
        return gcd(b, a % b)
    }
}

fn reduce(input: Fraction) -> Fraction {
    let divisor = gcd(input.numerator.abs(), input.denominator);
    Fraction { numerator: input.numerator / divisor, denominator: input.denominator / divisor }
}

fn process_input(input: &str) -> Result<String, String> {
    let mut elements = input.split_whitespace();
    let f1 = Fraction::from_str(elements.next().unwrap()).unwrap();
    let operator = elements.next().unwrap();
    let f2 = Fraction::from_str(elements.next().unwrap()).unwrap();

    let mut result = match operator {
        "*" => Fraction { numerator: f1.numerator * f2.numerator, denominator: f1.denominator * f2.denominator },
        "/" => Fraction { numerator: f1.numerator * f2.denominator, denominator: f1.denominator * f2.numerator },
        "+" => Fraction { numerator: (f1.numerator * f2.denominator) + (f2.numerator * f1.denominator), denominator: f1.denominator * f2.denominator},
        "-" => Fraction { numerator: (f1.numerator * f2.denominator) - (f2.numerator * f1.denominator), denominator: f1.denominator * f2.denominator},
        _ => return Err(String::from("Unknown operator"))
    };

    result = reduce(result);
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
    fn whole_number() {
        assert_eq!(
            Fraction::from_str("1").unwrap(),
            Fraction { numerator: 1, denominator: 1 });
        assert_eq!(
            Fraction::from_str("0").unwrap(),
            Fraction { numerator: 0, denominator: 1 });
        assert_eq!(
            Fraction::from_str("-1").unwrap(),
            Fraction { numerator: -1, denominator: 1 });
    }

    #[test]
    fn fraction() {
        assert_eq!(
            Fraction::from_str("1/4").unwrap(),
            Fraction { numerator: 1, denominator: 4 });
        assert_eq!(
            Fraction::from_str("-1/4").unwrap(),
            Fraction { numerator: -1, denominator: 4 });
    }

    #[test]
    fn mixed() {
        assert_eq!(
            Fraction::from_str("1_1/4").unwrap(),
            Fraction { numerator: 5, denominator: 4 });
        assert_eq!(
            Fraction::from_str("-1_1/4").unwrap(),
            Fraction { numerator: -5, denominator: 4 });
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", Fraction { numerator: 1, denominator: 1 }), "1");
        assert_eq!(format!("{}", Fraction { numerator: -1, denominator: 1 }), "-1");
        assert_eq!(format!("{}", Fraction { numerator: 0, denominator: 1 }), "0");
        assert_eq!(format!("{}", Fraction { numerator: 1, denominator: 4 }), "1/4");
        assert_eq!(format!("{}", Fraction { numerator: -1, denominator: 4 }), "-1/4");
        assert_eq!(format!("{}", Fraction { numerator: 5, denominator: 4 }), "1_1/4");
        assert_eq!(format!("{}", Fraction { numerator: -5, denominator: 4 }), "-1_1/4");
    }

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
