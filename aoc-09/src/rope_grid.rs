use crate::rope_movement::RopeMovement;
use aoc_core::cardinal_direction::CardinalDirection;
use std::collections::HashSet;

type RopePoint = (isize, isize);

pub struct RopeGrid {
    rope: Vec<RopePoint>,
    pub tail_visited: HashSet<RopePoint>,
}

impl RopeGrid {
    pub fn new(size: usize) -> Self {
        if size < 2 {
            panic!("Rope must have at least two knots!");
        }
        let rope = Vec::from_iter((0..size).map(|_| (0, 0)));
        RopeGrid {
            rope,
            tail_visited: HashSet::new(),
        }
    }

    pub fn move_head(&mut self, movement: &RopeMovement) {
        for _ in 0..movement.dist {
            let head = &mut self.rope[0];
            match movement.dir {
                CardinalDirection::North => *head = (head.0, head.1 - 1),
                CardinalDirection::South => *head = (head.0, head.1 + 1),
                CardinalDirection::East => *head = (head.0 + 1, head.1),
                CardinalDirection::West => *head = (head.0 - 1, head.1),
            };
            for i in 1..self.rope.len() {
                self.move_tail(i);
            }
            self.tail_visited.insert(*self.rope.last().unwrap());
        }
    }

    fn move_tail(&mut self, index: usize) {
        let head = &self.rope[index - 1];
        let tail = &self.rope[index];
        let x_dist = head.0 - tail.0;
        let y_dist = head.1 - tail.1;
        if x_dist.abs() == 2 && y_dist.abs() == 2 {
            self.rope[index] = (tail.0 + x_dist / 2, tail.1 + y_dist / 2)
        } else if x_dist.abs() == 2 {
            self.rope[index] = (tail.0 + x_dist / 2, head.1)
        } else if y_dist.abs() == 2 {
            self.rope[index] = (head.0, tail.1 + y_dist / 2)
        }
    }
}

impl Default for RopeGrid {
    fn default() -> Self {
        RopeGrid::new(2)
    }
}
