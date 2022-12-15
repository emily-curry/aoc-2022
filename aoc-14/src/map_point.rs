#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MapPoint {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for MapPoint {
    fn from(input: &str) -> Self {
        let mut split = input.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        MapPoint { x, y }
    }
}

impl Default for MapPoint {
    fn default() -> Self {
        MapPoint { x: 500, y: 0 }
    }
}
