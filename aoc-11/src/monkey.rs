use crate::monkey_decision::MonkeyDecision;
use crate::monkey_expression::MonkeyExpression;
use crate::monkey_item::MonkeyItem;

pub struct Monkey {
    pub items: Vec<MonkeyItem>,
    pub inspect_count: usize,
    worry_expression: MonkeyExpression,
    pub decision: MonkeyDecision,
}

impl Monkey {
    pub fn perform_turn(&mut self, lcm: Option<usize>) -> Vec<(MonkeyItem, usize)> {
        let mut result = vec![];
        for mut item in self.items.drain(0..) {
            self.worry_expression.evaluate_worry(&mut item, lcm);
            self.inspect_count += 1;

            let target = self.decision.decide_target(&item);
            result.push((item, target));
        }
        result
    }

    pub fn catch(&mut self, item: MonkeyItem) {
        self.items.push(item);
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let lines = &mut input.lines();
        let items = lines
            .skip(1)
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap().into())
            .collect();
        let worry_expression = lines.next().unwrap().into();
        let decision = lines.into();

        Monkey {
            items,
            worry_expression,
            inspect_count: 0,
            decision,
        }
    }
}
