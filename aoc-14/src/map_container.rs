use crate::map_content::MapContent;
use crate::map_line::MapLine;
use crate::map_point::MapPoint;
use std::collections::HashMap;
use std::str::Lines;

pub struct MapContainer {
    map: HashMap<MapPoint, MapContent>,
    abyss_y: usize,
    assume_infinite: bool,
}

impl MapContainer {
    pub fn fill(&mut self) {
        while let Some(point) = self.step() {
            let result = self.map.insert(point, MapContent::Sand);
            debug_assert_eq!(result, None);
        }
    }

    pub fn count_sand(&self) -> usize {
        self.map
            .values()
            .filter(|c| matches!(c, MapContent::Sand))
            .count()
    }

    pub fn set_infinite(&mut self, value: bool) {
        self.assume_infinite = value;
    }

    fn step(&self) -> Option<MapPoint> {
        let mut sand = MapPoint::default();
        if self.map.contains_key(&sand) {
            return None;
        }
        loop {
            if self.assume_infinite && sand.y >= self.abyss_y {
                return None;
            }
            sand.y += 1;
            if self.is_open(&sand) {
                continue;
            }
            sand.x -= 1;
            if self.is_open(&sand) {
                continue;
            }
            sand.x += 2;
            if self.is_open(&sand) {
                continue;
            }
            sand.x -= 1;
            sand.y -= 1;
            return Some(sand);
        }
    }

    fn is_open(&self, point: &MapPoint) -> bool {
        if point.y > self.abyss_y + 1 {
            false
        } else {
            !self.map.contains_key(point)
        }
    }
}

impl From<Lines<'_>> for MapContainer {
    fn from(input: Lines<'_>) -> Self {
        let mut map: HashMap<MapPoint, MapContent> = HashMap::new();
        let mut abyss_y = 0usize;
        for line in input.map(MapLine::from) {
            for point in line.iter_points() {
                if point.y > abyss_y {
                    abyss_y = point.y;
                }
                map.insert(point, MapContent::Rock);
            }
        }
        MapContainer {
            map,
            abyss_y,
            assume_infinite: true,
        }
    }
}
