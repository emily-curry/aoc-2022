use crate::monkey::Monkey;

pub struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl MonkeyGroup {
    pub fn perform_round(&mut self, reduce_worry: bool) {
        let lcm = match reduce_worry {
            true => None,
            false => Some(self.lcm),
        };
        for i in 0..self.monkeys.len() {
            let thrown_items = self.monkeys[i].perform_turn(lcm);
            for (item, target) in thrown_items {
                self.monkeys[target].catch(item);
            }
        }
    }

    pub fn most_active_score(&self) -> usize {
        let mut copy: Vec<&Monkey> = self.monkeys.iter().collect();
        copy.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
        copy.iter()
            .take(2)
            .fold(1usize, |acc, val| acc * val.inspect_count)
    }
}

impl From<&str> for MonkeyGroup {
    fn from(input: &str) -> Self {
        let monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
        let lcm = monkeys.iter().map(|x| x.decision.value).product();

        MonkeyGroup { monkeys, lcm }
    }
}
