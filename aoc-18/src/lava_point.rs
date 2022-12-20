use std::iter::from_fn;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LavaPoint {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl LavaPoint {
    pub fn iter_adj(&self) -> impl Iterator<Item = LavaPoint> + '_ {
        let mut step = 0;
        from_fn(move || {
            let r = match step {
                0 => Some(self.copy_with(|s| s.x += 1)),
                1 => Some(self.copy_with(|s| s.x -= 1)),
                2 => Some(self.copy_with(|s| s.y += 1)),
                3 => Some(self.copy_with(|s| s.y -= 1)),
                4 => Some(self.copy_with(|s| s.z += 1)),
                5 => Some(self.copy_with(|s| s.z -= 1)),
                _ => None,
            };
            step += 1;
            r
        })
    }

    pub fn is_adj(&self, other: &Self) -> bool {
        self.is_adj_x(other) || self.is_adj_y(other) || self.is_adj_z(other)
    }

    fn is_adj_x(&self, other: &Self) -> bool {
        self.y == other.y && self.z == other.z && self.x.abs_diff(other.x) == 1
    }

    fn is_adj_y(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z && self.y.abs_diff(other.y) == 1
    }

    fn is_adj_z(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x && self.z.abs_diff(other.z) == 1
    }

    fn copy_with<T>(&self, f: T) -> Self
    where
        T: Fn(&mut Self),
    {
        let mut copy = *self;
        f(&mut copy);
        copy
    }
}

impl From<&str> for LavaPoint {
    fn from(input: &str) -> Self {
        let mut split = input.split(',').map(|s| s.parse().unwrap());
        LavaPoint {
            x: split.next().unwrap(),
            y: split.next().unwrap(),
            z: split.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lava_point::LavaPoint;

    #[test]
    fn adj_x() {
        let a = LavaPoint::from("1,1,1");
        let b = LavaPoint::from("2,1,1");
        let c = LavaPoint::from("1,2,1");
        let d = LavaPoint::from("1,1,2");
        let e = LavaPoint::from("3,1,1");

        assert!(a.is_adj_x(&b));
        assert!(b.is_adj_x(&a));
        assert!(!a.is_adj_x(&c));
        assert!(!a.is_adj_x(&d));
        assert!(!a.is_adj_x(&e));
        assert!(b.is_adj_x(&e));
    }

    #[test]
    fn adj_y() {
        let a = LavaPoint::from("1,1,1");
        let b = LavaPoint::from("2,1,1");
        let c = LavaPoint::from("1,2,1");
        let d = LavaPoint::from("1,1,2");
        let e = LavaPoint::from("1,3,1");

        assert!(!a.is_adj_y(&b));
        assert!(a.is_adj_y(&c));
        assert!(!a.is_adj_y(&d));
        assert!(!a.is_adj_y(&e));
        assert!(c.is_adj_y(&e));
    }

    #[test]
    fn adj_z() {
        let a = LavaPoint::from("1,1,1");
        let b = LavaPoint::from("2,1,1");
        let c = LavaPoint::from("1,2,1");
        let d = LavaPoint::from("1,1,2");
        let e = LavaPoint::from("1,1,3");

        assert!(!a.is_adj_z(&b));
        assert!(!a.is_adj_z(&c));
        assert!(a.is_adj_z(&d));
        assert!(!a.is_adj_z(&e));
        assert!(d.is_adj_z(&e));
    }
}
