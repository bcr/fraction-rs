use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // sign_adjustment handles the case where the denominator is negative.
        // I'm sorry for that. But basically you just flip the sign of both
        // the numerator and denominator and let us never speak of that again.
        let sign_adjustment = if self.denominator < 0 { -1 } else { 1 };

        let numerator = (self.numerator * sign_adjustment).abs();
        let denominator = self.denominator * sign_adjustment;
        let is_negative = (self.numerator * sign_adjustment) < 0;

        if is_negative {
            write!(f, "-")?;
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
                write!(f, "{}_", whole)?;
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
            whole = remaining_string.parse::<i32>()?;
        }
    
        if has_whole {
            let mut elements = remaining_string.split("_");
            whole = elements.next().unwrap().parse::<i32>()?;
            remaining_string = elements.next().unwrap();
        }
    
        if has_fraction {
            let mut elements = remaining_string.split("/");
            numerator = elements.next().unwrap().parse::<i32>()?;
            denominator = elements.next().unwrap().parse::<i32>()?;
        }
    
        if whole < 0 {
            numerator = -numerator;
        }
    
        Ok(Fraction { numerator: (whole * denominator) + numerator, denominator: denominator })
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { numerator: (self.numerator * other.denominator) + (other.numerator * self.denominator), denominator: self.denominator * other.denominator }
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { numerator: (self.numerator * other.denominator) - (other.numerator * self.denominator), denominator: self.denominator * other.denominator}
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { numerator: self.numerator * other.numerator, denominator: self.denominator * other.denominator }
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self { numerator: self.numerator * other.denominator, denominator: self.denominator * other.numerator }
    }
}

impl Fraction {
    // https://stackoverflow.com/questions/18541832/c-sharp-find-the-greatest-common-divisor
    fn gcd(mut a : i32, mut b : i32) -> i32 {
        loop {
            if b == 0 {
                return a
            } else {
                let c = b;
                b = a % b;
                a = c;
            }
        }
    }

    pub fn reduce(input: Fraction) -> Result<Fraction, String> {
        let divisor = Fraction::gcd(input.numerator.abs(), input.denominator);
        Ok(Fraction { numerator: input.numerator / divisor, denominator: input.denominator / divisor })
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
        assert_eq!(format!("{}", Fraction { numerator: 1, denominator: -4 }), "-1/4");
        assert_eq!(format!("{}", Fraction { numerator: 5, denominator: 4 }), "1_1/4");
        assert_eq!(format!("{}", Fraction { numerator: -5, denominator: 4 }), "-1_1/4");
    }
}
