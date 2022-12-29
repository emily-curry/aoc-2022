use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use crate::monkey::{Monkey, BALD_MONKEY};
use crate::monkey_expression::MonkeyExpression;
use crate::monkey_operator::MonkeyOperator;

#[derive(Debug, Clone, Eq, PartialEq)]
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
        exp.reduce()
    }

    pub fn solve(self, eq_to: Self) -> isize {
        println!("Solve: {} == {}", self, eq_to);
        match eq_to {
            MonkeyExpressionTreeNode::Variable => {
                let reduced = self.reduce();
                if let MonkeyExpressionTreeNode::Value(result) = reduced {
                    result
                } else {
                    panic!(
                        "RHS of equation was only a variable, but could not reduce the LHS: {}",
                        reduced
                    )
                }
            }
            MonkeyExpressionTreeNode::Value(_) => {
                panic!("RHS should never be a plain value")
            }
            MonkeyExpressionTreeNode::Expression(rhs_exp) => match rhs_exp.op {
                MonkeyOperator::Add => {
                    let (rhs_value, inner_exp) = rhs_exp.unwrap_either_value().unwrap();
                    (self - MonkeyExpressionTreeNode::Value(rhs_value)).solve(inner_exp)
                }
                MonkeyOperator::Sub => {
                    if matches!(rhs_exp.rhs, MonkeyExpressionTreeNode::Value(_)) {
                        (self + rhs_exp.rhs).solve(rhs_exp.lhs)
                    } else if matches!(rhs_exp.lhs, MonkeyExpressionTreeNode::Value(_)) {
                        ((self - rhs_exp.lhs) * MonkeyExpressionTreeNode::Value(-1))
                            .solve(rhs_exp.rhs)
                    } else {
                        panic!("Solve sub: One side must be a value")
                    }
                }
                MonkeyOperator::Mul => {
                    let (rhs_value, inner_exp) = rhs_exp.unwrap_either_value().unwrap();
                    (self / MonkeyExpressionTreeNode::Value(rhs_value)).solve(inner_exp)
                }
                MonkeyOperator::Div => {
                    if matches!(rhs_exp.rhs, MonkeyExpressionTreeNode::Value(_)) {
                        (self * rhs_exp.rhs).solve(rhs_exp.lhs)
                    } else if matches!(rhs_exp.lhs, MonkeyExpressionTreeNode::Value(_)) {
                        panic!("Never actually occurs in input :)))")
                    } else {
                        panic!("Solve div: One side must be a value")
                    }
                }
            },
        }
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

impl Add for MonkeyExpressionTreeNode {
    type Output = MonkeyExpressionTreeNode;

    fn add(self, rhs: Self) -> Self::Output {
        let next = MonkeyExpressionTree {
            lhs: self,
            rhs,
            op: MonkeyOperator::Add,
        };
        MonkeyExpressionTreeNode::Expression(Box::new(next))
    }
}

impl Sub for MonkeyExpressionTreeNode {
    type Output = MonkeyExpressionTreeNode;

    fn sub(self, rhs: Self) -> Self::Output {
        let next = MonkeyExpressionTree {
            lhs: self,
            rhs,
            op: MonkeyOperator::Sub,
        };
        MonkeyExpressionTreeNode::Expression(Box::new(next))
    }
}

impl Mul for MonkeyExpressionTreeNode {
    type Output = MonkeyExpressionTreeNode;

    fn mul(self, rhs: Self) -> Self::Output {
        let next = MonkeyExpressionTree {
            lhs: self,
            rhs,
            op: MonkeyOperator::Mul,
        };
        MonkeyExpressionTreeNode::Expression(Box::new(next))
    }
}

impl Div for MonkeyExpressionTreeNode {
    type Output = MonkeyExpressionTreeNode;

    fn div(self, rhs: Self) -> Self::Output {
        let next = MonkeyExpressionTree {
            lhs: self,
            rhs,
            op: MonkeyOperator::Div,
        };
        MonkeyExpressionTreeNode::Expression(Box::new(next))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

    fn unwrap_either_value(self) -> Option<(isize, MonkeyExpressionTreeNode)> {
        if let MonkeyExpressionTreeNode::Value(v) = &self.lhs {
            Some((*v, self.rhs))
        } else if let MonkeyExpressionTreeNode::Value(v) = &self.rhs {
            Some((*v, self.lhs))
        } else {
            None
        }
    }
}

impl Display for MonkeyExpressionTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} {} {})", self.lhs, self.op, self.rhs))
    }
}
