use crate::monkey_map::MonkeyMapPoint;
use crate::monkey_map_instruction::MonkeyMapTurn;
use aoc_core::cardinal_direction::CardinalDirection;

pub struct MonkeyMapState {
    position: MonkeyMapPoint,
    facing: CardinalDirection,
}

impl MonkeyMapState {
    pub fn new(initial_position: MonkeyMapPoint) -> Self {
        MonkeyMapState {
            position: initial_position,
            facing: CardinalDirection::East,
        }
    }

    pub fn turn(&mut self, dir: &MonkeyMapTurn) {
        self.facing = match dir {
            MonkeyMapTurn::Left => match &self.facing {
                CardinalDirection::North => CardinalDirection::West,
                CardinalDirection::West => CardinalDirection::South,
                CardinalDirection::South => CardinalDirection::East,
                CardinalDirection::East => CardinalDirection::North,
            },
            MonkeyMapTurn::Right => match &self.facing {
                CardinalDirection::North => CardinalDirection::East,
                CardinalDirection::East => CardinalDirection::South,
                CardinalDirection::South => CardinalDirection::West,
                CardinalDirection::West => CardinalDirection::North,
            },
        };
    }

    pub fn get_position(&self) -> &MonkeyMapPoint {
        &self.position
    }

    pub fn set_position(&mut self, position: MonkeyMapPoint) {
        self.position = position;
    }

    pub fn get_facing(&self) -> &CardinalDirection {
        &self.facing
    }

    pub fn set_facing(&mut self, facing: CardinalDirection) {
        self.facing = facing;
    }

    pub fn get_password(&self) -> usize {
        let (x, y) = self.position;
        let facing_score: usize = match &self.facing {
            CardinalDirection::East => 0,
            CardinalDirection::South => 1,
            CardinalDirection::West => 2,
            CardinalDirection::North => 3,
        };
        ((y + 1) * 1000) + ((x + 1) * 4) + facing_score
    }
}
