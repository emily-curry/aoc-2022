use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MonkeyOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl MonkeyOperator {
    pub fn eval(&self, lhs: &isize, rhs: &isize) -> Option<isize> {
        match self {
            MonkeyOperator::Add => Some(lhs + rhs),
            MonkeyOperator::Sub => Some(lhs - rhs),
            MonkeyOperator::Mul => Some(lhs * rhs),
            MonkeyOperator::Div => match lhs % rhs {
                0 => Some(lhs / rhs),
                _ => None,
            },
        }
    }
}

impl From<&str> for MonkeyOperator {
    fn from(input: &str) -> Self {
        input.chars().next().unwrap().into()
    }
}

impl From<char> for MonkeyOperator {
    fn from(input: char) -> Self {
        match input {
            '+' => MonkeyOperator::Add,
            '-' => MonkeyOperator::Sub,
            '*' => MonkeyOperator::Mul,
            '/' => MonkeyOperator::Div,
            _ => panic!(),
        }
    }
}

impl Display for MonkeyOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            MonkeyOperator::Add => '+',
            MonkeyOperator::Sub => '-',
            MonkeyOperator::Mul => '*',
            MonkeyOperator::Div => '/',
        };
        f.write_char(c)
    }
}
