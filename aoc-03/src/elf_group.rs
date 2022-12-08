use crate::rucksack::Rucksack;
use crate::rucksack_item::RucksackItem;

pub struct ElfGroup {
    rucksacks: [Rucksack; 3],
}

impl ElfGroup {
    pub fn sum_misplaced_items(&self) -> u64 {
        self.rucksacks.iter().fold(0, |acc, val| {
            acc + val.find_misplaced_item().get_priority() as u64
        })
    }

    pub fn find_badge(&self) -> RucksackItem {
        for item in self.rucksacks[0].iter() {
            if self.rucksacks[1].contains(item) && self.rucksacks[2].contains(item) {
                return *item;
            }
        }
        panic!("No suitable badge found!")
    }
}

impl From<&[&str]> for ElfGroup {
    fn from(input: &[&str]) -> Self {
        let rucksacks = [
            Rucksack::from(input[0]),
            Rucksack::from(input[1]),
            Rucksack::from(input[2]),
        ];
        ElfGroup { rucksacks }
    }
}
