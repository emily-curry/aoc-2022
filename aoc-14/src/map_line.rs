use crate::map_point::MapPoint;

pub struct MapLine {
    points: Vec<MapPoint>,
}

impl MapLine {
    pub fn iter_points(&self) -> impl Iterator<Item = MapPoint> + '_ {
        self.points.windows(2).flat_map(|win| {
            let start = win[0];
            let end = win[1];
            (start.x.min(end.x)..=start.x.max(end.x)).flat_map(move |x| {
                (start.y.min(end.y)..=start.y.max(end.y)).map(move |y| MapPoint { x, y })
            })
        })
    }
}

impl From<&str> for MapLine {
    fn from(input: &str) -> Self {
        let points = input.split(" -> ").map(MapPoint::from).collect();
        MapLine { points }
    }
}
