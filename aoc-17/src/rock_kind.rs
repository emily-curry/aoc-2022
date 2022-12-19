use crate::rock_group::RockGroup;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RockKind {
    Dash,
    Cross,
    Bracket,
    Pole,
    Square,
}

impl RockKind {
    pub fn to_rock(&self, max_y: usize) -> RockGroup {
        let bottom = max_y + 4;
        let points = match self {
            RockKind::Dash => vec![(2, bottom), (3, bottom), (4, bottom), (5, bottom)],
            RockKind::Cross => vec![
                (3, bottom),
                (2, bottom + 1),
                (3, bottom + 1),
                (4, bottom + 1),
                (3, bottom + 2),
            ],
            RockKind::Bracket => vec![
                (2, bottom),
                (3, bottom),
                (4, bottom),
                (4, bottom + 1),
                (4, bottom + 2),
            ],
            RockKind::Pole => vec![
                (2, bottom),
                (2, bottom + 1),
                (2, bottom + 2),
                (2, bottom + 3),
            ],
            RockKind::Square => vec![(2, bottom), (2, bottom + 1), (3, bottom), (3, bottom + 1)],
        };
        RockGroup::new(points)
    }
}
