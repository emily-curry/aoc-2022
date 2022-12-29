use std::fmt::{Display, Formatter, Write};
use std::iter::Sum;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnafuNumber {
    digits: Vec<i8>,
}

impl SnafuNumber {
    pub fn to_decimal(&self) -> i64 {
        self.digits
            .iter()
            .enumerate()
            .map(|(p, d)| *d as i64 * (5i64.pow(p as u32)))
            .sum()
    }
}

impl From<&str> for SnafuNumber {
    fn from(input: &str) -> Self {
        let mut digits = Vec::new();
        for c in input.chars().rev() {
            let d: i8 = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _a => panic!("Not a valid SNAFU digit: {}", _a),
            };
            digits.push(d);
        }

        SnafuNumber { digits }
    }
}

impl From<i64> for SnafuNumber {
    fn from(input: i64) -> Self {
        let mut num = SnafuNumber {
            digits: Vec::from([2]),
        };
        while num.to_decimal() < input.abs() {
            num.digits.push(2);
        }
        for d in num.digits.iter_mut() {
            *d = 0;
        }
        for i in (0..num.digits.len()).rev() {
            for sl in [2i8, 1, 0, -1, -2].windows(2) {
                num.digits[i] = sl[0];
                let l_diff = num.to_decimal().abs_diff(input);
                num.digits[i] = sl[1];
                let r_diff = num.to_decimal().abs_diff(input);
                if l_diff < r_diff {
                    num.digits[i] = sl[0];
                    break;
                }
            }
        }

        num
    }
}

impl Add for SnafuNumber {
    type Output = SnafuNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SnafuNumber::from(self.to_decimal() + rhs.to_decimal())
    }
}

impl<'a> Sum<&'a Self> for SnafuNumber {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let decimal: i64 = iter.map(|n| n.to_decimal()).sum();
        SnafuNumber::from(decimal)
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            let c = match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _a => panic!("Not a valid SNAFU digit: {}", _a),
            };
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::snafu_number::SnafuNumber;

    fn get_example() -> Vec<SnafuNumber> {
        r"1=-0-2,12111,2=0=,21,2=01,111,20012,112,1=-1=,1-12,12,1=,122"
            .split(',')
            .map(|l| SnafuNumber::from(l))
            .collect()
    }

    fn get_decimals() -> Vec<i64> {
        r"1747,906,198,11,201,31,1257,32,353,107,7,3,37"
            .split(',')
            .map(|l| l.parse().unwrap())
            .collect()
    }

    #[test]
    fn to_decimal() {
        let decimals = get_decimals();
        for (i, snafu) in get_example().iter().enumerate() {
            assert_eq!(snafu.to_decimal(), decimals[i]);
        }
    }

    #[test]
    fn from_decimal() {
        let example = get_example();
        for (i, dec) in get_decimals().iter().enumerate() {
            assert_eq!(SnafuNumber::from(*dec), example[i]);
        }
    }

    #[test]
    fn example1() {
        let input = get_example();
        let sum: SnafuNumber = input.iter().sum();
        assert_eq!(sum.to_decimal(), 4890);
        assert_eq!(sum.digits, vec![0, -2, 1, -1, -2, 2]);
        assert_eq!(format_args!("{}", sum).to_string().as_str(), "2=-1=0");
    }
}
