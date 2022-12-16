#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct MapPoint {
    pub x: isize,
    pub y: isize,
}

impl MapPoint {
    pub fn new(x: isize, y: isize) -> Self {
        MapPoint { x, y }
    }

    pub fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<&str> for MapPoint {
    fn from(input: &str) -> Self {
        let mut split = input.split(", ");
        let x = split.next().unwrap().replace("x=", "").parse().unwrap();
        let y = split.next().unwrap().replace("y=", "").parse().unwrap();
        MapPoint { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::map_point::MapPoint;

    #[test]
    fn dist() {
        let a = MapPoint::new(0, 6);
        let b = MapPoint::new(0, -6);
        let c = MapPoint::new(-6, 0);
        let d = MapPoint::new(6, 0);
        assert_eq!(a.dist(&b), 12);
        assert_eq!(b.dist(&a), 12);
        assert_eq!(a.dist(&c), 12);
        assert_eq!(a.dist(&d), 12);
        assert_eq!(b.dist(&c), 12);
        assert_eq!(c.dist(&d), 12);

        assert_eq!(a.dist(&a), 0);
    }
}
