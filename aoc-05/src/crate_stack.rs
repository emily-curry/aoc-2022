use crate::crate_item::CrateItem;

pub struct CrateStack {
    crates: Vec<CrateItem>,
}

impl CrateStack {
    pub fn size(&self) -> usize {
        self.crates.len()
    }

    pub fn get(&self, index: usize) -> Option<&CrateItem> {
        self.crates.get(index)
    }

    pub fn peek(&self) -> Option<&CrateItem> {
        self.crates.last()
    }

    pub fn pop(&mut self) -> Option<CrateItem> {
        self.crates.pop()
    }

    pub fn push(&mut self, item: CrateItem) {
        self.crates.push(item);
    }
}

impl From<Vec<CrateItem>> for CrateStack {
    fn from(input: Vec<CrateItem>) -> Self {
        CrateStack { crates: input }
    }
}
