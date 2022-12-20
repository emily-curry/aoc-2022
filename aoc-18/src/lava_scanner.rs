use crate::lava_point::LavaPoint;
use std::collections::HashSet;
use std::str::Lines;

pub struct LavaScanner {
    points: Vec<LavaPoint>,
    enclosed: HashSet<LavaPoint>,
}

impl LavaScanner {
    pub fn get_surface_area(&self) -> usize {
        let mut surface_area = self.points.len() * 6;
        for i in 0..(self.points.len() - 1) {
            let ip = &self.points[i];
            for k in (i + 1)..self.points.len() {
                let kp = &self.points[k];
                if ip.is_adj(&kp) {
                    surface_area -= 2;
                }
            }
        }
        surface_area
    }

    pub fn get_external_surface_area(&self) -> usize {
        let mut surface_area = self.get_surface_area();
        for p in self.points.iter() {
            let air_points = p.iter_adj().filter(|ap| !self.points.contains(ap));
            for ap in air_points {
                if self.enclosed.contains(&ap) {
                    surface_area -= 1;
                }
            }
        }

        surface_area
    }

    fn find_enclosed(points: &Vec<LavaPoint>) -> HashSet<LavaPoint> {
        let mut enclosed: HashSet<LavaPoint> = points
            .iter()
            .flat_map(|p| p.iter_adj())
            .filter(|p| !points.contains(p))
            .collect();
        for p in enclosed.clone().iter() {
            if !LavaScanner::is_enclosed(points, p) {
                LavaScanner::remove_recursive(&mut enclosed, p);
            }
        }
        enclosed
    }

    fn is_enclosed(points: &Vec<LavaPoint>, point: &LavaPoint) -> bool {
        let has_left = points
            .iter()
            .any(|p| p.y == point.y && p.z == point.z && p.x < point.x);
        let has_right = points
            .iter()
            .any(|p| p.y == point.y && p.z == point.z && p.x > point.x);
        let has_forward = points
            .iter()
            .any(|p| p.x == point.x && p.z == point.z && p.y < point.y);
        let has_backward = points
            .iter()
            .any(|p| p.x == point.x && p.z == point.z && p.y > point.y);
        let has_up = points
            .iter()
            .any(|p| p.y == point.y && p.x == point.x && p.z < point.z);
        let has_down = points
            .iter()
            .any(|p| p.y == point.y && p.x == point.x && p.z > point.z);
        has_left && has_right && has_forward && has_backward && has_up && has_down
    }

    fn remove_recursive(enclosed: &mut HashSet<LavaPoint>, point: &LavaPoint) {
        let removed = enclosed.remove(point);
        if removed {
            for adj in point.iter_adj() {
                LavaScanner::remove_recursive(enclosed, &adj);
            }
        }
    }
}

impl From<Lines<'_>> for LavaScanner {
    fn from(input: Lines<'_>) -> Self {
        let points: Vec<LavaPoint> = input.map(LavaPoint::from).collect();
        let enclosed = LavaScanner::find_enclosed(&points);

        LavaScanner { points, enclosed }
    }
}

#[cfg(test)]
mod tests {
    use crate::lava_scanner::LavaScanner;

    fn get_input() -> LavaScanner {
        let input = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        LavaScanner::from(input.lines())
    }

    #[test]
    fn example() {
        let input = get_input();
        let surface_area = input.get_surface_area();
        assert_eq!(surface_area, 64);
    }

    #[test]
    fn example2() {
        let input = get_input();
        let surface_area = input.get_external_surface_area();
        assert_eq!(surface_area, 58);
    }
}
