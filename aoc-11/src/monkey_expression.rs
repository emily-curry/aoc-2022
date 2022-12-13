use crate::monkey_item::MonkeyItem;

pub struct MonkeyExpression {
    lhs: MonkeyOperand,
    rhs: MonkeyOperand,
    op: MonkeyOperator,
}

impl MonkeyExpression {
    pub fn evaluate_worry(&self, item: &mut MonkeyItem, lcm_opt: Option<usize>) {
        item.value = self.op.evaluate(
            self.lhs.get_value(&item.value),
            self.rhs.get_value(&item.value),
        );

        if let Some(lcm) = lcm_opt {
            item.value = item.value % lcm
        } else {
            item.value = (item.value - (item.value % 3)) / 3;
        }
    }
}

impl From<&str> for MonkeyExpression {
    fn from(input: &str) -> Self {
        let mut split = input.split("= ").skip(1).next().unwrap().split(' ');
        let lhs = split.next().unwrap().into();
        let op = split.next().unwrap().into();
        let rhs = split.next().unwrap().into();
        MonkeyExpression { lhs, op, rhs }
    }
}

pub enum MonkeyOperand {
    Old,
    Value(usize),
}

impl MonkeyOperand {
    pub fn get_value(&self, item: &usize) -> usize {
        match self {
            MonkeyOperand::Old => *item,
            MonkeyOperand::Value(v) => *v,
        }
    }
}

impl From<&str> for MonkeyOperand {
    fn from(input: &str) -> Self {
        match input {
            "old" => MonkeyOperand::Old,
            _ => MonkeyOperand::Value(input.parse().unwrap()),
        }
    }
}

pub enum MonkeyOperator {
    Add,
    Mul,
}

impl From<&str> for MonkeyOperator {
    fn from(input: &str) -> Self {
        match input {
            "*" => MonkeyOperator::Mul,
            "+" => MonkeyOperator::Add,
            _a => panic!("{} is not an operator!", _a),
        }
    }
}

impl MonkeyOperator {
    pub fn evaluate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            MonkeyOperator::Add => lhs + rhs,
            MonkeyOperator::Mul => lhs * rhs,
        }
    }
}
