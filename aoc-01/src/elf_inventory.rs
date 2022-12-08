use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};

#[derive(PartialEq, Eq)]
pub struct ElfInventory {
    food: Vec<u64>,
}

impl ElfInventory {
    pub fn sum_calories(&self) -> u64 {
        self.food.iter().fold(0, |acc, val| acc + val)
    }
}

impl From<&str> for ElfInventory {
    fn from(input: &str) -> Self {
        let food = input.split("\n").map(|x| x.parse().unwrap()).collect();
        ElfInventory { food }
    }
}

impl PartialOrd for ElfInventory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sum_calories().partial_cmp(&other.sum_calories())
    }
}

impl Ord for ElfInventory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum_calories().cmp(&other.sum_calories())
    }
}

impl Display for ElfInventory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("> ")?;
        for item in &self.food {
            f.write_fmt(format_args!("{} ", item))?;
        }
        f.write_fmt(format_args!(">> {}", self.sum_calories()))?;
        Ok(())
    }
}
