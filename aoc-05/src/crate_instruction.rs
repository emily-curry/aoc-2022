pub struct CrateInstruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for CrateInstruction {
    fn from(input: &str) -> Self {
        let mut values = input
            .split(' ')
            .map(str::parse::<usize>)
            .filter(Result::is_ok)
            .map(Result::unwrap);
        let count = values.next().unwrap();
        let from = values.next().unwrap() - 1;
        let to = values.next().unwrap() - 1;
        CrateInstruction { count, from, to }
    }
}
