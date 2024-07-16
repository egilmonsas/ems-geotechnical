#![allow(dead_code)]
#![warn(clippy::pedantic)]

pub mod hydro;
pub mod profile;
pub mod soil;

#[cfg(test)]
mod tests {
    use crate::{
        hydro::ProfilePorePressure,
        profile::{Point, Profile},
    };

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
        approx::assert_abs_diff_eq!(result, expected);
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
