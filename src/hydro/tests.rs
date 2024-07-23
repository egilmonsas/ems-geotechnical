use crate::profile::Point;

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
