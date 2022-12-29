use aoc_core::cardinal_direction::CardinalDirection;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

pub type BlizzardMapPoint = (usize, usize);

#[derive(Clone, Debug)]
pub struct BlizzardMap {
    points: BTreeSet<(BlizzardMapPoint, CardinalDirection)>,
    points_only: BTreeSet<BlizzardMapPoint>,
    step: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl BlizzardMap {
    pub fn get_next(&self) -> Self {
        let mut next_points: BTreeSet<(BlizzardMapPoint, CardinalDirection)> = BTreeSet::new();
        for (point, dir) in self.points.iter() {
            let next_point = match dir {
                CardinalDirection::North => {
                    if point.1 == 1 {
                        (point.0, self.max_y - 1)
                    } else {
                        (point.0, point.1 - 1)
                    }
                }
                CardinalDirection::South => {
                    if point.1 == self.max_y - 1 {
                        (point.0, 1)
                    } else {
                        (point.0, point.1 + 1)
                    }
                }
                CardinalDirection::West => {
                    if point.0 == 1 {
                        (self.max_x - 1, point.1)
                    } else {
                        (point.0 - 1, point.1)
                    }
                }
                CardinalDirection::East => {
                    if point.0 == self.max_x - 1 {
                        (1, point.1)
                    } else {
                        (point.0 + 1, point.1)
                    }
                }
            };
            next_points.insert((next_point, *dir));
        }
        let points_only = next_points.iter().map(|(p, _)| *p).collect();

        BlizzardMap {
            points: next_points,
            points_only,
            step: self.step + 1,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    pub fn contains(&self, point: &BlizzardMapPoint) -> bool {
        self.points_only.contains(point)
    }
}

impl From<&str> for BlizzardMap {
    fn from(input: &str) -> Self {
        let mut points = BTreeSet::new();
        let mut max_x = 0usize;
        let mut max_y = 0usize;
        for (y, line) in input.lines().enumerate() {
            if y > max_y {
                max_y = y;
            }
            for (x, c) in line.chars().enumerate() {
                if x > max_x {
                    max_x = x;
                }
                let dir = match c {
                    '<' => Some(CardinalDirection::West),
                    '>' => Some(CardinalDirection::East),
                    '^' => Some(CardinalDirection::North),
                    'v' => Some(CardinalDirection::South),
                    _ => None,
                };
                if let Some(d) = dir {
                    points.insert(((x, y), d));
                }
            }
        }
        let points_only = points.iter().map(|(p, _)| *p).collect();

        BlizzardMap {
            points,
            points_only,
            step: 0,
            max_x,
            max_y,
        }
    }
}

impl PartialEq for BlizzardMap {
    fn eq(&self, other: &Self) -> bool {
        self.points.eq(&other.points)
    }
}

impl Eq for BlizzardMap {}

impl Hash for BlizzardMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.points.hash(state);
    }
}

