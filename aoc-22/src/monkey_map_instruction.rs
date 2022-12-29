#[derive(Debug, Copy, Clone)]
pub enum MonkeyMapTurn {
    Left,
    Right,
}

impl From<char> for MonkeyMapTurn {
    fn from(input: char) -> Self {
        match input {
            'L' => MonkeyMapTurn::Left,
            'R' => MonkeyMapTurn::Right,
            _ => panic!("Cannot turn neither left nor right"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MonkeyMapInstruction {
    Turn(MonkeyMapTurn),
    Advance(usize),
}
