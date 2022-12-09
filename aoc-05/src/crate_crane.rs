use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::str::Lines;

use crate::crate_instruction::CrateInstruction;
use crate::crate_item::CrateItem;
use crate::crate_stack::CrateStack;

pub enum CraneType {
    Single,
    Stack,
}

pub struct CrateCrane {
    stacks: [CrateStack; 9],
    instructions: Vec<CrateInstruction>,
}

impl CrateCrane {
    pub fn process_instructions(&mut self, crane_type: CraneType) {
        let process = match crane_type {
            CraneType::Single => CrateCrane::process_single,
            CraneType::Stack => CrateCrane::process_stack,
        };
        while !self.instructions.is_empty() {
            let inst = self.instructions.pop().unwrap();
            process(self, &inst);
        }
    }

    fn process_single(&mut self, inst: &CrateInstruction) {
        for _ in 0..inst.count {
            let item = self.stacks[inst.from].pop().unwrap();
            self.stacks[inst.to].push(item);
        }
    }

    fn process_stack(&mut self, inst: &CrateInstruction) {
        let mut grabbed: Vec<CrateItem> = vec![];
        for _ in 0..inst.count {
            let item = self.stacks[inst.from].pop().unwrap();
            grabbed.push(item);
        }
        grabbed.reverse();
        for item in grabbed {
            self.stacks[inst.to].push(item);
        }
    }

    pub fn top_code(&self) -> String {
        self.stacks
            .iter()
            .map(|x| {
                x.peek()
                    .map(|c| format_args!("{}", c).to_string())
                    .unwrap_or(String::from(" "))
            })
            .fold(String::new(), |acc, val| acc.add(&val))
    }
}

impl Display for CrateCrane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_size = self.stacks.iter().map(CrateStack::size).max().unwrap();
        for line_num in (0..max_size).rev() {
            for stack in &self.stacks {
                let item = stack.get(line_num);
                if item.is_none() {
                    f.write_str("   ")?;
                } else {
                    f.write_fmt(format_args!("[{}]", item.unwrap()))?;
                }
                f.write_str(" ")?;
            }
            f.write_str("\n")?;
        }
        for i in 0..self.stacks.len() {
            f.write_fmt(format_args!(" {}  ", i + 1))?;
        }
        Ok(())
    }
}

impl From<Lines<'_>> for CrateCrane {
    fn from(mut input: Lines<'_>) -> Self {
        let mut stack_items: [Vec<CrateItem>; 9] = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let stack_lines = input.by_ref().take_while(|l| l.contains('['));
        for line in stack_lines {
            let mut chars = line.chars();
            for i in 0..9usize {
                let char = chars.by_ref().skip(1).next().unwrap();
                if char != ' ' {
                    stack_items[i].push(CrateItem::from(char));
                }
                chars.by_ref().take(2).count();
            }
        }
        let stacks = stack_items.map(|mut items| {
            items.reverse();
            CrateStack::from(items)
        });

        input.next();
        let instructions: Vec<CrateInstruction> = input.rev().map(CrateInstruction::from).collect();

        CrateCrane {
            instructions,
            stacks,
        }
    }
}
