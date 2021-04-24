#[derive(PartialEq)]
#[derive(Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32
}

fn parse_fraction(fraction_string : &str) -> Fraction {
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

    Fraction { numerator: (whole * denominator) + numerator, denominator: denominator }
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

    #[test]
    fn fraction() {
        assert_eq!(
            parse_fraction("1/4"),
            Fraction { numerator: 1, denominator: 4 });
        assert_eq!(
            parse_fraction("-1/4"),
            Fraction { numerator: -1, denominator: 4 });
    }

    #[test]
    fn mixed() {
        assert_eq!(
            parse_fraction("1_1/4"),
            Fraction { numerator: 5, denominator: 4 });
        assert_eq!(
            parse_fraction("-1_1/4"),
            Fraction { numerator: -5, denominator: 4 });
    }
}
