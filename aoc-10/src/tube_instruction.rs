pub enum TubeInstruction {
    NoOp,
    AddX(isize),
}

impl TubeInstruction {
    pub fn cycles(&self) -> usize {
        match self {
            TubeInstruction::NoOp => 1,
            TubeInstruction::AddX(_) => 2,
        }
    }
}

impl From<&str> for TubeInstruction {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        match split.next().unwrap() {
            "addx" => TubeInstruction::AddX(split.next().unwrap().parse().unwrap()),
            "noop" => TubeInstruction::NoOp,
            _a => panic!("Not a valid command: {}", _a),
        }
    }
}
