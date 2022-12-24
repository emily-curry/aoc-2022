use crate::monkey::Monkey;
use crate::monkey_operator::MonkeyOperator;

#[derive(Debug, Copy, Clone)]
pub struct MonkeyExpression {
    pub lhs: Monkey,
    pub rhs: Monkey,
    pub op: MonkeyOperator,
}

impl MonkeyExpression {
    pub fn eval(&self, lhs: &isize, rhs: &isize) -> isize {
        self.op.eval(lhs, rhs).unwrap()
    }
}

impl From<&str> for MonkeyExpression {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        let lhs = split.next().unwrap().into();
        let op = split.next().unwrap().into();
        let rhs = split.next().unwrap().into();

        MonkeyExpression { lhs, op, rhs }
    }
}
