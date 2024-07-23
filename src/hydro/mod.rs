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
    pub fn drawdown_profile(&self, d_u_0: f64) -> Self {
        const INFLUENCE_DEPTH: f64 = 10.0;
        const DZ: f64 = 0.1;
        let total_depth = self.points.last().unwrap().x;
        let mut new_points = vec![];
        let mut z = 0.0;
        while z < total_depth {
            let u_0 = self.eval(z);

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

#[cfg(test)]
mod tests;
