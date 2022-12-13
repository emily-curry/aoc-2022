use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::str::Lines;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HeightMapTile {
    height: u8,
    x: usize,
    y: usize,
    is_start: bool,
    is_end: bool,
}

impl HeightMapTile {
    pub fn can_move(&self, to: &Self) -> bool {
        to.height <= self.height + 1
    }
}

pub struct HeightMapStep {
    pub step: usize,
    x: usize,
    y: usize,
}

impl HeightMapStep {
    pub fn from_tile(tile: &HeightMapTile, step: usize) -> Self {
        HeightMapStep {
            step,
            x: tile.x,
            y: tile.y,
        }
    }
}

impl PartialEq for HeightMapStep {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Eq for HeightMapStep {}

impl PartialOrd for HeightMapStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeightMapStep {
    fn cmp(&self, other: &Self) -> Ordering {
        other.step.cmp(&self.step)
    }
}

pub struct HeightMap {
    map: Vec<Vec<HeightMapTile>>,
}

impl HeightMap {
    pub fn find_shortest_path(&self, fuzzy_start: bool) -> HeightMapStep {
        let mut visited: HashSet<&HeightMapTile> = HashSet::new();
        let mut queue: BinaryHeap<HeightMapStep> = BinaryHeap::new();
        let end = self.find_end();
        visited.insert(&end);
        queue.push(HeightMapStep::from_tile(end, 0));
        let start = self.find_start();

        while let Some(next) = queue.pop() {
            if fuzzy_start {
                let next_tile = self.map[next.y][next.x];
                if next_tile.height == 'a' as u8 {
                    return next; // Part 2
                }
            } else {
                if next.x == start.x && next.y == start.y {
                    return next; // Part 1
                }
            }
            for adj in self.find_adjacent(&next).iter() {
                if visited.contains(adj) {
                    continue;
                }
                visited.insert(adj);
                let step = HeightMapStep::from_tile(adj, next.step + 1);
                queue.push(step);
            }
        }
        panic!()
    }

    fn find_adjacent(&self, step: &HeightMapStep) -> Vec<&HeightMapTile> {
        let current = &self.map[step.y][step.x];
        let mut result: Vec<&HeightMapTile> = Vec::new();
        if step.y != 0 {
            let next = &self.map[step.y - 1][step.x];
            if next.can_move(current) {
                result.push(next);
            }
        }
        if step.y != self.map.len() - 1 {
            let next = &self.map[step.y + 1][step.x];
            if next.can_move(current) {
                result.push(next);
            }
        }
        if step.x != 0 {
            let next = &self.map[step.y][step.x - 1];
            if next.can_move(current) {
                result.push(next);
            }
        }
        if step.x != self.map[step.y].len() - 1 {
            let next = &self.map[step.y][step.x + 1];
            if next.can_move(current) {
                result.push(next);
            }
        }
        result
    }

    fn find_end(&self) -> &HeightMapTile {
        self.map
            .iter()
            .flat_map(|y| y.iter())
            .find(|t| t.is_end)
            .unwrap()
    }

    fn find_start(&self) -> &HeightMapTile {
        self.map
            .iter()
            .flat_map(|y| y.iter())
            .find(|t| t.is_start)
            .unwrap()
    }
}

impl From<Lines<'_>> for HeightMap {
    fn from(input: Lines<'_>) -> Self {
        let map = input
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let mut is_start = false;
                        let mut is_end = false;
                        let height = match ch {
                            'S' => {
                                is_start = true;
                                'a' as u8
                            }
                            'E' => {
                                is_end = true;
                                'z' as u8
                            }
                            rest => rest as u8,
                        };
                        HeightMapTile {
                            height,
                            x,
                            y,
                            is_start,
                            is_end,
                        }
                    })
                    .collect()
            })
            .collect();

        HeightMap { map }
    }
}
