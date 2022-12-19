use crate::rock_point::RockPoint;

#[derive(Debug)]
pub struct RockGroup {
    pub points: Vec<RockPoint>,
}

impl RockGroup {
    pub fn new(points: Vec<RockPoint>) -> Self {
        RockGroup { points }
    }
}
