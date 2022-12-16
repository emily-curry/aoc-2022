use crate::map_point::MapPoint;
use crate::map_sensor::MapSensor;
use aoc_core::includes::Includes;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::Lines;

pub struct DistressMap {
    points: HashSet<MapSensor>,
}

const R: RangeInclusive<isize> = 0..=4000000;

impl DistressMap {
    pub fn count_excluded_in_row(&self, y: isize) -> usize {
        self.get_range_x()
            .map(|x| MapPoint::new(x, y))
            .filter(|p| {
                self.points
                    .iter()
                    .any(|s| s.includes(p) && &s.loc != p && &s.beacon != p)
            })
            .count()
    }

    pub fn find_distress_signal(&self) -> MapPoint {
        self.points
            .iter()
            .flat_map(|s| s.iter_bounds())
            .filter(|p| R.contains(&p.y) && R.contains(&p.x))
            .find(|p| self.points.iter().all(|s| !s.includes(p)))
            .unwrap()
    }

    fn get_range_x(&self) -> RangeInclusive<isize> {
        let min = self
            .points
            .iter()
            .flat_map(|p| [p.loc.x - p.dist() as isize, p.beacon.x])
            .min()
            .unwrap();
        let max = self
            .points
            .iter()
            .flat_map(|p| [p.loc.x + p.dist() as isize, p.beacon.x])
            .max()
            .unwrap();
        min..=max
    }
}

impl From<Lines<'_>> for DistressMap {
    fn from(input: Lines<'_>) -> Self {
        let points = input.map(MapSensor::from).collect();
        DistressMap { points }
    }
}
