#![allow(dead_code)]
pub mod points;
pub mod profile;
pub mod soil;

use points::*;
use profile::Profile;

#[derive(Debug, Clone)]
pub struct ProfilePorePressure {
    points: Vec<Point>,
}

impl Default for ProfilePorePressure {
    fn default() -> Self {
        Self {
            points: vec![Point::new(0.0, 0.0), Point::new(100.0, 1000.0)],
        }
    }
}

impl ProfilePorePressure {
    pub fn new(points: Vec<Point>) -> Self {
        let mut copy = points;
        copy.sort();
        Self { points: copy }
    }

    pub fn drawdown_profile(&self, d_u_0: f64) -> Self {
        const influence_depth: f64 = 10.0;
        const DZ: f64 = 0.1;
        let total_depth = self.points.last().unwrap().x();
        let mut new_points = vec![];
        let mut z = 0.0;
        while z < total_depth {
            let u_0 = self.eval(z);

            if z >= total_depth - influence_depth {
                let elapsed_depth = total_depth - z;
                let d_u = d_u_0 * (influence_depth - elapsed_depth) / influence_depth;
                let u = u_0 + d_u;

                new_points.push(Point::new(z, u.max(0.0)))
            } else {
                new_points.push(Point::new(z, u_0))
            }

            z += DZ;
        }
        ProfilePorePressure::new(new_points)
    }
}

impl Profile for ProfilePorePressure {
    fn new(points: Vec<Point>) -> Self {
        ProfilePorePressure::new(points)
    }

    fn points(&self) -> &Vec<Point> {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_profile() {
        let points = vec![
            Point::new(1.0, 0.0),
            Point::new(10.0, 90.0),
            Point::new(5.0, 40.0),
        ];
        let profile = ProfilePorePressure::new(points);

        dbg!(profile);
    }

    #[rstest]
    #[case(vec![Point::new(0.0, 0.0),Point::new(10.0,100.0)],-20.0, 0.0)]
    #[case(vec![Point::new(0.0, 0.0),Point::new(10.0,100.0)],0.0, 0.0)]
    #[case(vec![Point::new(0.0, 0.0),Point::new(10.0,100.0)],5.0, 50.0)]
    #[case(vec![Point::new(0.0, 0.0),Point::new(10.0,100.0)],10.0, 100.0)]
    #[case(vec![Point::new(0.0, 0.0),Point::new(10.0,100.0)],20.0, 100.0)]
    // Confirm sorting works
    #[case(vec![Point::new(10.0, 100.0),Point::new(0.0,0.0)],5.0, 50.0)]

    fn eval(#[case] points: Vec<Point>, #[case] eval_point: f64, #[case] expected: f64) {
        let profile = ProfilePorePressure::new(points);
        let result = profile.eval(eval_point);
        approx::assert_abs_diff_eq!(result, expected)
    }
    #[test]
    fn create_profile2() {
        let points = vec![
            Point::new(1.0, 0.0),
            Point::new(10.0, 90.0),
            Point::new(5.0, 40.0),
        ];
        let profile = ProfilePorePressure::new(points).drawdown_profile(-10.0);

        dbg!(profile);
    }
}
