use approx::assert_relative_eq;
use rstest::rstest;

use crate::profile::{Point, Profile};

use super::ProfilePorePressure;

#[test]
fn default() {
    ProfilePorePressure::default();
}

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

#[rstest]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),-10.0,0.5,4.5)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),-10.0,0.0,0.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),-10.0,4.5,40.5)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),-10.0,10.0,90.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),10.0,0.5,5.5)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),10.0,0.0,0.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),10.0,4.5,49.5)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),10.0,10.0,110.0)]

fn drawdown(
    #[case] profile: ProfilePorePressure,
    #[case] drawdown_amount: f64,
    #[case] eval_depth: f64,
    #[case] expexted: f64,
) {
    let drawdown_profile = ProfilePorePressure::drawdown_profile(&profile, drawdown_amount);
    assert_relative_eq!(drawdown_profile.eval(eval_depth), expexted);
}

#[rstest]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),10.0,0.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),0.0,0.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0)]),0.0,0.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0)]),10.0,100.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0),Point::new(5.0, 10.0), Point::new(10.0, 100.0)]),5.0,40.0)]
#[case(ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]),ProfilePorePressure::new(vec![Point::new(0.0, 0.0),Point::new(5.0, 10.0), Point::new(10.0, 100.0)]),7.5,20.0)]

fn subtraction(
    #[case] profile: ProfilePorePressure,
    #[case] rhs: ProfilePorePressure,
    #[case] eval_depth: f64,
    #[case] expexted: f64,
) {
    assert_relative_eq!((profile - rhs).eval(eval_depth), expexted);
}

#[test]
fn subtraction_dbg() {
    let lhs = ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]);
    let rhs = ProfilePorePressure::new(vec![Point::new(0.0, 0.0), Point::new(10.0, 100.0)]);
    dbg!(lhs - rhs);
}
