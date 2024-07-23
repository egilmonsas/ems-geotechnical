use cgmath::Vector2;

use crate::profile::Point;
use crate::profile::Profile;

#[derive(Debug, Clone)]
pub struct ProfilePorePressure {
    points: Vec<Vector2<f64>>,
}

impl Default for ProfilePorePressure {
    fn default() -> Self {
        Self {
            points: vec![Point::new(0.0, 0.0), Point::new(100.0, 1000.0)],
        }
    }
}

impl ProfilePorePressure {
    #[must_use]
    pub fn new(points: Vec<Point>) -> Self {
        let mut copy = points;
        copy.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        Self { points: copy }
    }
    /// # Panics
    /// Will panic if pointslist is empty
    #[must_use]
    pub fn drawdown_profile(origin_profile: &Self, d_u_0: f64) -> Self {
        const INFLUENCE_DEPTH: f64 = 10.0;
        const DZ: f64 = 0.5;
        let total_depth = origin_profile.points.last().unwrap().x;
        let mut new_points = vec![];
        let mut z = 0.0;
        while z <= total_depth {
            let u_0 = origin_profile.eval(z);

            if z >= total_depth - INFLUENCE_DEPTH {
                let elapsed_depth = total_depth - z;
                let d_u = d_u_0 * (INFLUENCE_DEPTH - elapsed_depth) / INFLUENCE_DEPTH;
                let u = u_0 + d_u;

                new_points.push(Point::new(z, u.max(0.0)));
            } else {
                new_points.push(Point::new(z, u_0));
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

impl std::ops::Sub<Self> for ProfilePorePressure {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let xs = self
            .points
            .iter()
            .chain(rhs.points.iter())
            .map(|p| p.x)
            .collect::<Vec<f64>>();

        Self::new(
            xs.iter()
                .map(|&x| Point::new(x, self.eval(x) - rhs.eval(x)))
                .collect::<Vec<Point>>(),
        )
    }
}
