use std::collections::{HashMap, HashSet};
use std::iter::from_fn;
use std::str::Lines;

pub type ElfMapPoint = (isize, isize);

pub struct ElfMap {
    elves: HashSet<ElfMapPoint>,
    current_step: u32,
}

impl ElfMap {
    pub fn step(&mut self) -> bool {
        let proposed_moves = self.collect_propositions();
        let executed = self.execute_moves(&proposed_moves);
        self.current_step += 1;
        executed > 0
    }

    pub fn count_empty(&self) -> usize {
        let min_x = self.elves.iter().map(|p| p.0).min().unwrap();
        let max_x = self.elves.iter().map(|p| p.0).max().unwrap();
        let min_y = self.elves.iter().map(|p| p.1).min().unwrap();
        let max_y = self.elves.iter().map(|p| p.1).max().unwrap();
        let x = max_x.abs_diff(min_x) + 1;
        let y = max_y.abs_diff(min_y) + 1;
        (x * y) - self.elves.len()
    }

    pub fn get_current_step(&self) -> u32 {
        self.current_step
    }

    fn collect_propositions(&self) -> HashMap<ElfMapPoint, ElfMapPoint> {
        let mut result = HashMap::new();
        for point in self.elves.iter() {
            if ElfMap::iter_around(point).all(|p| !self.elves.contains(&p)) {
                continue;
            }
            for i in 0..4u32 {
                match (i + self.current_step) % 4 {
                    0 => {
                        if ElfMap::iter_north(point).all(|p| !self.elves.contains(&p)) {
                            result.insert(*point, (point.0, point.1 - 1));
                            break;
                        }
                    }
                    1 => {
                        if ElfMap::iter_south(point).all(|p| !self.elves.contains(&p)) {
                            result.insert(*point, (point.0, point.1 + 1));
                            break;
                        }
                    }
                    2 => {
                        if ElfMap::iter_west(point).all(|p| !self.elves.contains(&p)) {
                            result.insert(*point, (point.0 - 1, point.1));
                            break;
                        }
                    }
                    3 => {
                        if ElfMap::iter_east(point).all(|p| !self.elves.contains(&p)) {
                            result.insert(*point, (point.0 + 1, point.1));
                            break;
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        result
    }

    fn execute_moves(&mut self, moves: &HashMap<ElfMapPoint, ElfMapPoint>) -> u32 {
        let mut proposed_counts: HashMap<ElfMapPoint, u32> = HashMap::new();
        let mut moves_executed = 0u32;
        for target in moves.values() {
            *proposed_counts.entry(*target).or_insert(0) += 1;
        }
        let allowed: HashSet<ElfMapPoint> = proposed_counts
            .iter()
            .filter(|(_, v)| **v == 1)
            .map(|(k, _)| *k)
            .collect();
        for (current, target) in moves.iter() {
            if allowed.contains(target) {
                self.elves.remove(current);
                self.elves.insert(*target);
                moves_executed += 1;
            }
        }
        moves_executed
    }

    fn iter_around(point: &ElfMapPoint) -> impl Iterator<Item = ElfMapPoint> + '_ {
        let mut i = 0u8;
        from_fn(move || {
            let r = match i {
                0 => Some((point.0 - 1, point.1 - 1)),
                1 => Some((point.0, point.1 - 1)),
                2 => Some((point.0 + 1, point.1 - 1)),
                3 => Some((point.0 - 1, point.1 + 1)),
                4 => Some((point.0, point.1 + 1)),
                5 => Some((point.0 + 1, point.1 + 1)),
                6 => Some((point.0 - 1, point.1)),
                7 => Some((point.0 + 1, point.1)),
                _ => None,
            };
            i += 1;
            r
        })
    }

    fn iter_north(point: &ElfMapPoint) -> impl Iterator<Item = ElfMapPoint> + '_ {
        let mut i = 0u8;
        from_fn(move || {
            let r = match i {
                0 => Some((point.0 - 1, point.1 - 1)),
                1 => Some((point.0, point.1 - 1)),
                2 => Some((point.0 + 1, point.1 - 1)),
                _ => None,
            };
            i += 1;
            r
        })
    }

    fn iter_south(point: &ElfMapPoint) -> impl Iterator<Item = ElfMapPoint> + '_ {
        let mut i = 0u8;
        from_fn(move || {
            let r = match i {
                0 => Some((point.0 - 1, point.1 + 1)),
                1 => Some((point.0, point.1 + 1)),
                2 => Some((point.0 + 1, point.1 + 1)),
                _ => None,
            };
            i += 1;
            r
        })
    }

    fn iter_east(point: &ElfMapPoint) -> impl Iterator<Item = ElfMapPoint> + '_ {
        let mut i = 0u8;
        from_fn(move || {
            let r = match i {
                0 => Some((point.0 + 1, point.1 - 1)),
                1 => Some((point.0 + 1, point.1)),
                2 => Some((point.0 + 1, point.1 + 1)),
                _ => None,
            };
            i += 1;
            r
        })
    }

    fn iter_west(point: &ElfMapPoint) -> impl Iterator<Item = ElfMapPoint> + '_ {
        let mut i = 0u8;
        from_fn(move || {
            let r = match i {
                0 => Some((point.0 - 1, point.1 - 1)),
                1 => Some((point.0 - 1, point.1)),
                2 => Some((point.0 - 1, point.1 + 1)),
                _ => None,
            };
            i += 1;
            r
        })
    }
}

impl From<Lines<'_>> for ElfMap {
    fn from(input: Lines<'_>) -> Self {
        let mut elves = HashSet::new();
        for (y, line) in input.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }

        ElfMap {
            elves,
            current_step: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::elf_map::ElfMap;

    fn get_input() -> ElfMap {
        r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            .lines()
            .into()
    }

    #[test]
    fn example1() {
        let mut map = get_input();
        for _ in 0..10 {
            map.step();
        }
        assert_eq!(map.count_empty(), 110);
    }

    #[test]
    fn example2() {
        let mut map = get_input();
        while map.step() {}
        assert_eq!(map.get_current_step(), 20);
    }
}
