use crate::map_point::MapPoint;
use aoc_core::includes::Includes;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MapSensor {
    pub loc: MapPoint,
    pub beacon: MapPoint,
}

impl MapSensor {
    #[allow(dead_code)]
    pub fn new(loc: MapPoint, beacon: MapPoint) -> Self {
        MapSensor { loc, beacon }
    }

    pub fn dist(&self) -> usize {
        self.loc.dist(&self.beacon)
    }

    pub fn iter_bounds(&self) -> impl Iterator<Item = MapPoint> {
        let target = self.dist() as isize + 1;
        let lower = self.loc.x - target;
        let upper = self.loc.x + target;
        let y = self.loc.y.clone();
        (lower..=self.loc.x).flat_map(move |x| {
            let dy = lower.abs_diff(x) as isize;
            let a = MapPoint::new(x, y + dy);
            let b = MapPoint::new(x, y - dy);
            let c = MapPoint::new(upper - dy, y + dy);
            let d = MapPoint::new(upper - dy, y - dy);
            [a, b, c, d]
        })
    }
}

impl Includes<MapPoint> for MapSensor {
    fn includes(&self, other: &MapPoint) -> bool {
        let beacon_dist = self.dist();
        let point_dist = self.loc.dist(other);
        point_dist <= beacon_dist
    }
}

impl From<&str> for MapSensor {
    fn from(input: &str) -> Self {
        let mut split = input.split("at ");
        split.next();
        let loc = split
            .next()
            .unwrap()
            .split(':')
            .next()
            .map(MapPoint::from)
            .unwrap();
        let beacon = split.next().map(MapPoint::from).unwrap();
        MapSensor { loc, beacon }
    }
}

#[cfg(test)]
mod tests {
    use crate::map_point::MapPoint;
    use crate::map_sensor::MapSensor;
    use aoc_core::includes::Includes;
    use std::collections::HashSet;

    #[test]
    fn dist() {
        let s = MapSensor::new(MapPoint::new(0, 0), MapPoint::new(6, 6));
        assert_eq!(s.dist(), 12);
    }

    #[test]
    fn includes() {
        let s = MapSensor::new(MapPoint::new(0, 0), MapPoint::new(6, 6));
        assert_eq!(s.includes(&MapPoint::new(0, 0)), true);
        assert_eq!(s.includes(&MapPoint::new(6, 6)), true);
        assert_eq!(s.includes(&MapPoint::new(3, 3)), true);
        assert_eq!(s.includes(&MapPoint::new(9, 3)), true);
        assert_eq!(s.includes(&MapPoint::new(-6, -6)), true);
        assert_eq!(s.includes(&MapPoint::new(-12, 0)), true);
        assert_eq!(s.includes(&MapPoint::new(0, 12)), true);

        assert_eq!(s.includes(&MapPoint::new(-13, 0)), false);
        assert_eq!(s.includes(&MapPoint::new(0, 13)), false);
    }

    #[test]
    fn iter_bounds() {
        let s = MapSensor::new(MapPoint::new(-1, 0), MapPoint::new(0, 1))
            .iter_bounds()
            .collect::<HashSet<MapPoint>>();
        let ex = HashSet::from([
            MapPoint::new(-4, 0),
            MapPoint::new(-3, 1),
            MapPoint::new(-3, -1),
            MapPoint::new(-2, 2),
            MapPoint::new(-2, -2),
            MapPoint::new(-1, 3),
            MapPoint::new(-1, -3),
            MapPoint::new(0, 2),
            MapPoint::new(0, -2),
            MapPoint::new(1, 1),
            MapPoint::new(1, -1),
            MapPoint::new(2, 0),
        ]);
        assert_eq!(s, ex);
    }
}
