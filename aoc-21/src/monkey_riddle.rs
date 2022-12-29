use crate::monkey::{Monkey, ROOT_MONKEY};
use crate::monkey_expression::MonkeyExpression;
use crate::monkey_expression_tree::MonkeyExpressionTreeNode;
use std::collections::{HashMap, VecDeque};
use std::str::Lines;

pub struct MonkeyRiddle {
    pending: VecDeque<(Monkey, MonkeyExpression)>,
    evaluated: HashMap<Monkey, isize>,
}

impl MonkeyRiddle {
    pub fn solve_for_root(&self) -> isize {
        let mut pending = self.pending.clone();
        let mut evaluated = self.evaluated.clone();

        while let Some((key, exp)) = pending.pop_front() {
            if let (Some(lhs), Some(rhs)) = (evaluated.get(&exp.lhs), evaluated.get(&exp.rhs)) {
                let result = exp.eval(lhs, rhs);
                evaluated.insert(key, result);
                if key == ROOT_MONKEY {
                    return result;
                }
            } else {
                pending.push_back((key, exp))
            }
        }
        panic!("No value for root ever found!");
    }

    pub fn solve_for_human(&self) -> isize {
        let mut expressions = self
            .pending
            .clone()
            .into_iter()
            .collect::<HashMap<Monkey, MonkeyExpression>>();
        let root = expressions.remove(&ROOT_MONKEY).unwrap();
        let lhs = MonkeyExpressionTreeNode::expand(&root.lhs, &expressions, &self.evaluated);
        let rhs = MonkeyExpressionTreeNode::expand(&root.rhs, &expressions, &self.evaluated);
        if matches!(lhs, MonkeyExpressionTreeNode::Value(_)) {
            lhs.solve(rhs)
        } else if matches!(rhs, MonkeyExpressionTreeNode::Value(_)) {
            rhs.solve(lhs)
        } else {
            panic!("One side must be a value to solve")
        }
    }
}

impl From<Lines<'_>> for MonkeyRiddle {
    fn from(input: Lines<'_>) -> Self {
        let mut pending: VecDeque<(Monkey, MonkeyExpression)> = VecDeque::new();
        let mut evaluated: HashMap<Monkey, isize> = HashMap::new();
        for line in input {
            let mut split = line.split(": ");
            let key = Monkey::from(split.next().unwrap());
            let value_str = split.next().unwrap();
            if value_str.contains(' ') {
                let value_exp = MonkeyExpression::from(value_str);
                pending.push_back((key, value_exp));
            } else {
                let value: isize = value_str.parse().unwrap();
                evaluated.insert(key, value);
            }
        }

        MonkeyRiddle { pending, evaluated }
    }
}

#[cfg(test)]
mod tests {
    use crate::monkey_riddle::MonkeyRiddle;

    fn get_input() -> MonkeyRiddle {
        let s = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        MonkeyRiddle::from(s.lines())
    }

    #[test]
    fn example1() {
        let riddle = get_input();
        assert_eq!(riddle.solve_for_root(), 152);
    }

    #[test]
    fn example2() {
        let riddle = get_input();
        assert_eq!(riddle.solve_for_human(), 301);
    }
}
