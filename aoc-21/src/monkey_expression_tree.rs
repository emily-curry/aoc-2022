use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::monkey::{Monkey, BALD_MONKEY};
use crate::monkey_expression::MonkeyExpression;
use crate::monkey_operator::MonkeyOperator;

#[derive(Debug, Clone)]
pub enum MonkeyExpressionTreeNode {
    Value(isize),
    Expression(Box<MonkeyExpressionTree>),
    Variable,
}

impl MonkeyExpressionTreeNode {
    pub fn expand(
        target: &Monkey,
        expressions: &HashMap<Monkey, MonkeyExpression>,
        values: &HashMap<Monkey, isize>,
    ) -> Self {
        if values.contains_key(target) {
            panic!("Cannot expand a constant value!");
        }
        if *target == BALD_MONKEY {
            panic!("Cannot expand our variable!")
        }
        let exp = expressions.get(target).unwrap();
        let lhs = MonkeyExpressionTree::make_node(&exp.lhs, expressions, values);
        let rhs = MonkeyExpressionTree::make_node(&exp.rhs, expressions, values);

        let exp = MonkeyExpressionTree {
            lhs,
            rhs,
            op: exp.op,
        };
        let node = MonkeyExpressionTreeNode::Expression(Box::new(exp));
        node.reduce()
    }

    pub fn solve_for_humn(self, eq_to: Self) -> isize {
        if let MonkeyExpressionTreeNode::Value(v) = self.solve(eq_to) {
            v
        } else {
            panic!("Did not find solution!");
        }
    }

    fn solve(self, eq_to: Self) -> Self {
        println!("{} == {}", eq_to, self);
        if matches!(eq_to, MonkeyExpressionTreeNode::Variable) {
            return self.reduce();
        }
        if let MonkeyExpressionTreeNode::Expression(rhs_exp) = eq_to {
            todo!()
        }

        panic!("I don't think this should happen")
    }

    fn reduce(self) -> Self {
        match self {
            MonkeyExpressionTreeNode::Value(_) => self,
            MonkeyExpressionTreeNode::Expression(exp) => exp.reduce(),
            MonkeyExpressionTreeNode::Variable => self,
        }
    }
}

impl Display for MonkeyExpressionTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyExpressionTreeNode::Value(value) => f.write_fmt(format_args!("{}", value)),
            MonkeyExpressionTreeNode::Variable => f.write_str("humn"),
            MonkeyExpressionTreeNode::Expression(exp) => exp.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyExpressionTree {
    lhs: MonkeyExpressionTreeNode,
    rhs: MonkeyExpressionTreeNode,
    op: MonkeyOperator,
}

impl MonkeyExpressionTree {
    fn reduce(self) -> MonkeyExpressionTreeNode {
        if let Some(v) = self.try_reduce_value() {
            return MonkeyExpressionTreeNode::Value(v);
        }

        MonkeyExpressionTreeNode::Expression(Box::new(self))
    }

    fn try_reduce_value(&self) -> Option<isize> {
        let reduced_lhs = match &self.lhs {
            MonkeyExpressionTreeNode::Value(value) => Some(*value),
            MonkeyExpressionTreeNode::Expression(exp) => exp.try_reduce_value(),
            _ => None,
        };
        let reduced_rhs = match &self.rhs {
            MonkeyExpressionTreeNode::Value(value) => Some(*value),
            MonkeyExpressionTreeNode::Expression(exp) => exp.try_reduce_value(),
            _ => None,
        };
        if let (Some(lhs), Some(rhs)) = (reduced_lhs, reduced_rhs) {
            return self.op.eval(&lhs, &rhs);
        }
        None
    }

    fn make_node(
        monkey: &Monkey,
        expressions: &HashMap<Monkey, MonkeyExpression>,
        values: &HashMap<Monkey, isize>,
    ) -> MonkeyExpressionTreeNode {
        if *monkey == BALD_MONKEY {
            MonkeyExpressionTreeNode::Variable
        } else if values.contains_key(monkey) {
            MonkeyExpressionTreeNode::Value(*values.get(monkey).unwrap())
        } else {
            MonkeyExpressionTreeNode::expand(monkey, expressions, values)
        }
    }
}

impl Display for MonkeyExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} {} {})", self.lhs, self.op, self.rhs))
    }
}
