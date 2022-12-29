use crate::monkey_map_tile::MonkeyMapTile;
use aoc_core::cardinal_direction::CardinalDirection;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

/// (x, y), index starts at 0
pub type MonkeyMapPoint = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum MonkeyMapFace {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

pub struct MonkeyMap {
    /// Indexed by row then column, i.e `tiles[y][x]`.
    tiles: Vec<Vec<MonkeyMapTile>>,
    face_relationships: HashMap<
        (MonkeyMapFace, CardinalDirection),
        Box<dyn Fn(&MonkeyMapPoint) -> (MonkeyMapPoint, CardinalDirection)>,
    >,
}

impl MonkeyMap {
    pub fn find_initial_point(&self) -> MonkeyMapPoint {
        for (x, col) in self.tiles[0].iter().enumerate() {
            if matches!(col, MonkeyMapTile::Open) {
                return (x, 0);
            }
        }
        panic!("No initial point found");
    }

    pub fn find_next_point(
        &self,
        position: &MonkeyMapPoint,
        facing: &CardinalDirection,
        steps: &usize,
        as_cube: bool,
    ) -> (MonkeyMapPoint, CardinalDirection) {
        let mut result_position = *position;
        let mut result_facing = *facing;
        for _ in 0..*steps {
            if let Some((next_position, next_facing)) =
                self.step(&result_position, &result_facing, as_cube)
            {
                result_position = next_position;
                result_facing = next_facing;
            } else {
                break;
            }
        }
        (result_position, result_facing)
    }

    fn step(
        &self,
        position: &MonkeyMapPoint,
        facing: &CardinalDirection,
        as_cube: bool,
    ) -> Option<(MonkeyMapPoint, CardinalDirection)> {
        let (x, y) = match facing {
            CardinalDirection::West => self.scan_row(position, true),
            CardinalDirection::East => self.scan_row(position, false),
            CardinalDirection::North => self.scan_col(position, false),
            CardinalDirection::South => self.scan_col(position, true),
        };
        if !as_cube {
            match self.tiles[y][x] {
                MonkeyMapTile::Open => Some(((x, y), *facing)),
                MonkeyMapTile::Wall => None,
                MonkeyMapTile::Void => panic!("Never step into the void!"),
            }
        } else {
            let current_face = self.get_face(position);
            let next_face = self.get_face(&self.get_point_wrapping(position, facing));
            let ((next_x, next_y), next_facing) = if current_face != next_face {
                self.face_relationships
                    .get(&(current_face.unwrap(), *facing))
                    .map(|f| f(position))
                    .unwrap_or_else(|| ((x, y), *facing))
            } else {
                ((x, y), *facing)
            };
            match self.tiles[next_y][next_x] {
                MonkeyMapTile::Open => Some(((next_x, next_y), next_facing)),
                MonkeyMapTile::Wall => None,
                MonkeyMapTile::Void => panic!("Never step into the void!"),
            }
        }
    }

    fn get_point_wrapping(
        &self,
        position: &MonkeyMapPoint,
        facing: &CardinalDirection,
    ) -> MonkeyMapPoint {
        match facing {
            CardinalDirection::West => {
                if position.0 == 0 {
                    (self.tiles[position.1].len() - 1, position.1)
                } else {
                    (position.0 - 1, position.1)
                }
            }
            CardinalDirection::East => {
                if position.0 == self.tiles[position.1].len() - 1 {
                    (0, position.1)
                } else {
                    (position.0 + 1, position.1)
                }
            }
            CardinalDirection::North => {
                if position.1 == 0 {
                    (position.0, self.tiles.len() - 1)
                } else {
                    (position.0, position.1 - 1)
                }
            }
            CardinalDirection::South => {
                if position.1 == self.tiles.len() - 1 {
                    (position.0, 0)
                } else {
                    (position.0, position.1 + 1)
                }
            }
        }
    }

    fn scan_row(&self, start_from: &MonkeyMapPoint, is_west: bool) -> MonkeyMapPoint {
        let facing = match is_west {
            true => CardinalDirection::West,
            false => CardinalDirection::East,
        };
        let mut result = self.get_point_wrapping(start_from, &facing);

        while matches!(self.tiles[result.1][result.0], MonkeyMapTile::Void) {
            result = self.get_point_wrapping(&result, &facing);
        }
        result
    }

    fn scan_col(&self, start_from: &MonkeyMapPoint, is_south: bool) -> MonkeyMapPoint {
        let facing = match is_south {
            true => CardinalDirection::South,
            false => CardinalDirection::North,
        };
        let mut result = self.get_point_wrapping(start_from, &facing);

        while matches!(self.tiles[result.1][result.0], MonkeyMapTile::Void) {
            result = self.get_point_wrapping(&result, &facing);
        }
        result
    }

    #[cfg(test)]
    fn get_face(&self, position: &MonkeyMapPoint) -> Option<MonkeyMapFace> {
        if position.1 < 4 {
            if (8..12).contains(&position.0) {
                return Some(MonkeyMapFace::Top);
            }
        } else if position.1 < 8 {
            if (0..4).contains(&position.0) {
                return Some(MonkeyMapFace::Front);
            } else if (4..8).contains(&position.0) {
                return Some(MonkeyMapFace::Left);
            } else if (8..12).contains(&position.0) {
                return Some(MonkeyMapFace::Back);
            }
        } else {
            if (8..12).contains(&position.0) {
                return Some(MonkeyMapFace::Bottom);
            } else if (12..16).contains(&position.0) {
                return Some(MonkeyMapFace::Right);
            }
        }

        None
    }

    #[cfg(test)]
    fn make_face_relationships() -> HashMap<
        (MonkeyMapFace, CardinalDirection),
        Box<dyn Fn(&MonkeyMapPoint) -> (MonkeyMapPoint, CardinalDirection)>,
    > {
        let mut result: HashMap<
            (MonkeyMapFace, CardinalDirection),
            Box<dyn Fn(&MonkeyMapPoint) -> (MonkeyMapPoint, CardinalDirection)>,
        > = HashMap::new();
        result.insert(
            (MonkeyMapFace::Top, CardinalDirection::North),
            Box::new(|p| ((3 - (p.0 % 4), 4), CardinalDirection::South)),
        );
        result.insert(
            (MonkeyMapFace::Top, CardinalDirection::West),
            Box::new(|p| ((4 + p.1, 4), CardinalDirection::South)),
        );
        result.insert(
            (MonkeyMapFace::Top, CardinalDirection::East),
            Box::new(|p| ((15, 11 - (p.1 % 4)), CardinalDirection::West)),
        );
        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::North),
            Box::new(|p| ((11 - (p.0 % 4), 0), CardinalDirection::South)),
        );
        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::West),
            Box::new(|p| ((15 - (p.1 % 4), 11), CardinalDirection::North)),
        );
        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::South),
            Box::new(|p| ((8 + (p.1 % 4), 11), CardinalDirection::North)),
        );
        result.insert(
            (MonkeyMapFace::Left, CardinalDirection::North),
            Box::new(|p| ((8, p.0 % 4), CardinalDirection::East)),
        );
        result.insert(
            (MonkeyMapFace::Left, CardinalDirection::South),
            Box::new(|p| ((8, 11 - (p.0 % 4)), CardinalDirection::East)),
        );
        result.insert(
            (MonkeyMapFace::Back, CardinalDirection::East),
            Box::new(|p| ((15 - (p.1 % 4), 8), CardinalDirection::South)),
        );
        result.insert(
            (MonkeyMapFace::Bottom, CardinalDirection::West),
            Box::new(|p| ((7 - (p.1 % 4), 7), CardinalDirection::North)),
        );
        result.insert(
            (MonkeyMapFace::Bottom, CardinalDirection::South),
            Box::new(|p| ((3 - (p.0 % 4), 7), CardinalDirection::North)),
        );
        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::North),
            Box::new(|p| ((11, 7 - (p.0 % 4)), CardinalDirection::West)),
        );
        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::East),
            Box::new(|p| ((11, 3 - (p.1 % 4)), CardinalDirection::West)),
        );
        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::South),
            Box::new(|p| ((0, 7 - (p.0 % 4)), CardinalDirection::East)),
        );

        result
    }

    #[cfg(not(test))]
    fn get_face(&self, position: &MonkeyMapPoint) -> Option<MonkeyMapFace> {
        if position.1 < 50 {
            if (50..100).contains(&position.0) {
                return Some(MonkeyMapFace::Top);
            } else if (100..150).contains(&position.0) {
                return Some(MonkeyMapFace::Right);
            }
        } else if position.1 < 100 {
            if (50..100).contains(&position.0) {
                return Some(MonkeyMapFace::Back);
            }
        } else if position.1 < 150 {
            if (0..50).contains(&position.0) {
                return Some(MonkeyMapFace::Left);
            } else if (50..100).contains(&position.0) {
                return Some(MonkeyMapFace::Bottom);
            }
        } else {
            if (0..50).contains(&position.0) {
                return Some(MonkeyMapFace::Front);
            }
        }

        None
    }

    #[cfg(not(test))]
    fn make_face_relationships() -> HashMap<
        (MonkeyMapFace, CardinalDirection),
        Box<dyn Fn(&MonkeyMapPoint) -> (MonkeyMapPoint, CardinalDirection)>,
    > {
        let mut result: HashMap<
            (MonkeyMapFace, CardinalDirection),
            Box<dyn Fn(&MonkeyMapPoint) -> (MonkeyMapPoint, CardinalDirection)>,
        > = HashMap::new();

        result.insert(
            (MonkeyMapFace::Top, CardinalDirection::North),
            Box::new(|p| ((0, 150 + (p.0 % 50)), CardinalDirection::East)),
        ); // top -> front
        result.insert(
            (MonkeyMapFace::Top, CardinalDirection::West),
            Box::new(|p| ((0, 149 - (p.1 % 50)), CardinalDirection::East)),
        ); // top -> left

        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::North),
            Box::new(|p| ((p.0 % 50, 199), CardinalDirection::North)),
        ); // right -> front
        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::East),
            Box::new(|p| ((99, 149 - (p.1 % 50)), CardinalDirection::West)),
        ); // right -> bottom
        result.insert(
            (MonkeyMapFace::Right, CardinalDirection::South),
            Box::new(|p| ((99, 50 + (p.0 % 50)), CardinalDirection::West)),
        ); // right -> back

        result.insert(
            (MonkeyMapFace::Back, CardinalDirection::East),
            Box::new(|p| ((100 + (p.1 % 50), 49), CardinalDirection::North)),
        ); // back -> right
        result.insert(
            (MonkeyMapFace::Back, CardinalDirection::West),
            Box::new(|p| ((p.1 % 50, 100), CardinalDirection::South)),
        ); // back -> left

        result.insert(
            (MonkeyMapFace::Bottom, CardinalDirection::East),
            Box::new(|p| ((149, 49 - (p.1 % 50)), CardinalDirection::West)),
        ); // bottom -> right
        result.insert(
            (MonkeyMapFace::Bottom, CardinalDirection::South),
            Box::new(|p| ((49, 150 + (p.0 % 50)), CardinalDirection::West)),
        ); // bottom -> front

        result.insert(
            (MonkeyMapFace::Left, CardinalDirection::West),
            Box::new(|p| ((50, 49 - (p.1 % 50)), CardinalDirection::East)),
        ); // left -> top
        result.insert(
            (MonkeyMapFace::Left, CardinalDirection::North),
            Box::new(|p| ((50, 50 + (p.0 % 50)), CardinalDirection::East)),
        ); // left -> back

        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::West),
            Box::new(|p| ((50 + (p.1 % 50), 0), CardinalDirection::South)),
        ); // front -> top
        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::South),
            Box::new(|p| ((100 + (p.0 % 50), 0), CardinalDirection::South)),
        ); // front -> right
        result.insert(
            (MonkeyMapFace::Front, CardinalDirection::East),
            Box::new(|p| ((50 + (p.1 % 50), 149), CardinalDirection::North)),
        ); // front -> bottom

        result
    }
}

impl<'a, T> From<T> for MonkeyMap
where
    T: Iterator<Item = &'a str>,
{
    fn from(lines: T) -> Self {
        let mut result: Vec<Vec<MonkeyMapTile>> = Vec::new();
        let mut max_length = 0usize;
        for row in lines {
            result.push(Vec::new());
            if row.len() > max_length {
                max_length = row.len();
            }
            let last = result.last_mut().unwrap();
            for col in row.chars() {
                last.push(col.into());
            }
        }
        for row in result.iter_mut() {
            while row.len() < max_length {
                row.push(MonkeyMapTile::Void);
            }
        }

        MonkeyMap {
            tiles: result,
            face_relationships: MonkeyMap::make_face_relationships(),
        }
    }
}

impl Display for MonkeyMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.tiles.iter().enumerate() {
            for col in row {
                col.fmt(f)?;
            }
            if i < self.tiles.len() - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::monkey_map::MonkeyMap;
    use aoc_core::cardinal_direction::CardinalDirection;
    fn get_input() -> String {
        r"        ...#    
        .#..    
        #...    
        ....    
...#.......#    
........#...    
..#....#....    
..........#.    
        ...#....
        .....#..
        .#......
        ......#."
            .to_string()
    }

    fn get_map() -> MonkeyMap {
        MonkeyMap::from(get_input().lines())
    }

    #[test]
    fn from() {
        let map = get_map();
        println!("{}", map);
        assert_eq!(get_input(), format_args!("{}", map).to_string());
    }

    #[test]
    fn face_relationships_top() {
        let map = get_map();
        let step = map.step(&(8, 0), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((4, 4), CardinalDirection::South));
        let step = map.step(&(8, 2), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((6, 4), CardinalDirection::South));
        let step = map.step(&(8, 0), &CardinalDirection::North, true);
        assert_eq!(step, None);
        let step = map.step(&(9, 0), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((2, 4), CardinalDirection::South));
        let step = map.step(&(11, 0), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((0, 4), CardinalDirection::South));
        let step = map.step(&(11, 0), &CardinalDirection::East, true).unwrap();
        assert_eq!(step, ((15, 11), CardinalDirection::West));
        let step = map.step(&(11, 3), &CardinalDirection::East, true).unwrap();
        assert_eq!(step, ((15, 8), CardinalDirection::West));
        let step = map.step(&(10, 3), &CardinalDirection::South, true).unwrap();
        assert_eq!(step, ((10, 4), CardinalDirection::South));
    }

    #[test]
    fn face_relationships_front() {
        let map = get_map();
        let step = map.step(&(2, 4), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((9, 0), CardinalDirection::South));
        let step = map.step(&(0, 7), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((12, 11), CardinalDirection::North));
        let step = map.step(&(0, 4), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((15, 11), CardinalDirection::North));
        let step = map.step(&(0, 7), &CardinalDirection::South, true).unwrap();
        assert_eq!(step, ((11, 11), CardinalDirection::North));
    }

    #[test]
    fn face_relationships_left() {
        let map = get_map();
        let step = map.step(&(4, 4), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((8, 0), CardinalDirection::East));
        let step = map.step(&(4, 7), &CardinalDirection::South, true).unwrap();
        assert_eq!(step, ((8, 11), CardinalDirection::East));
        let step = map.step(&(7, 7), &CardinalDirection::South, true).unwrap();
        assert_eq!(step, ((8, 8), CardinalDirection::East));
    }

    #[test]
    fn face_relationships_back() {
        let map = get_map();
        let step = map.step(&(11, 4), &CardinalDirection::East, true).unwrap();
        assert_eq!(step, ((15, 8), CardinalDirection::South));
        let step = map.step(&(11, 7), &CardinalDirection::East, true).unwrap();
        assert_eq!(step, ((12, 8), CardinalDirection::South));
    }

    #[test]
    fn face_relationships_bottom() {
        let map = get_map();
        let step = map.step(&(8, 8), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((7, 7), CardinalDirection::North));
        let step = map.step(&(8, 11), &CardinalDirection::West, true).unwrap();
        assert_eq!(step, ((4, 7), CardinalDirection::North));
        let step = map.step(&(8, 11), &CardinalDirection::South, true).unwrap();
        assert_eq!(step, ((3, 7), CardinalDirection::North));
        let step = map
            .step(&(11, 11), &CardinalDirection::South, true)
            .unwrap();
        assert_eq!(step, ((0, 7), CardinalDirection::North));
    }

    #[test]
    fn face_relationships_right() {
        let map = get_map();
        let step = map.step(&(12, 8), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((11, 7), CardinalDirection::West));
        let step = map.step(&(14, 8), &CardinalDirection::North, true).unwrap();
        assert_eq!(step, ((11, 5), CardinalDirection::West));
        let step = map.step(&(15, 8), &CardinalDirection::East, true).unwrap();
        assert_eq!(step, ((11, 3), CardinalDirection::West));
        let step = map
            .step(&(15, 11), &CardinalDirection::South, true)
            .unwrap();
        assert_eq!(step, ((0, 4), CardinalDirection::East));
    }
}
