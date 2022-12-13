use crate::monkey_item::MonkeyItem;

pub struct MonkeyDecision {
    pub value: usize,
    if_true: usize,
    if_false: usize,
}

impl MonkeyDecision {
    pub fn decide_target(&self, item: &MonkeyItem) -> usize {
        match item.value % self.value == 0 {
            true => self.if_true,
            false => self.if_false,
        }
    }
}

impl<'a, T> From<&mut T> for MonkeyDecision
where
    T: Iterator<Item = &'a str>,
{
    fn from(input: &mut T) -> Self {
        let value = input
            .next()
            .unwrap()
            .split("by ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let if_true = input
            .next()
            .unwrap()
            .split("monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let if_false = input
            .next()
            .unwrap()
            .split("monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        MonkeyDecision {
            value,
            if_true,
            if_false,
        }
    }
}
