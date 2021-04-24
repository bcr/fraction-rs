#[derive(PartialEq)]
#[derive(Debug)]
struct Fraction {
    numerator: i32,
    denominator: u32
}

fn parse_fraction(fraction_string : &str) -> Fraction {
    let whole = fraction_string.parse::<i32>().unwrap();
    Fraction { numerator: whole, denominator : 1}
}

// fn parse_operator(operator_string : &str) {
//     println!("Parsing operator {}", operator_string);
// }

// fn parse_input(input: &str) {
//     let mut elements = input.split_whitespace();
//     let f1 = parse_fraction(elements.next().unwrap());
//     let operator = parse_operator(elements.next().unwrap());
//     let f2 = parse_fraction(elements.next().unwrap());
// }

fn main() {
    // parse_input("1   +  ");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_number() {
        assert_eq!(
            parse_fraction("1"),
            Fraction { numerator: 1, denominator: 1 });
        assert_eq!(
            parse_fraction("0"),
            Fraction { numerator: 0, denominator: 1 });
        assert_eq!(
            parse_fraction("-1"),
            Fraction { numerator: -1, denominator: 1 });
    }
}
