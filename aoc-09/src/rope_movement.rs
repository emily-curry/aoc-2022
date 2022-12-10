use aoc_core::cardinal_direction::CardinalDirection;

pub struct RopeMovement {
    pub dir: CardinalDirection,
    pub dist: usize,
}

impl From<&str> for RopeMovement {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        let dir = split.next().unwrap().chars().next().unwrap().into();
        let dist = split.next().unwrap().parse::<usize>().unwrap();
        RopeMovement { dir, dist }
    }
}
