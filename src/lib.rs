mod points;
mod profile;
mod soil;

use points::*;
use profile::Profile;

#[derive(Debug)]
struct ProfilePorePressure {
    points: Vec<Point>,
}

impl ProfilePorePressure {
    fn new(points: Vec<Point>) -> Self {
        let mut copy = points;
        copy.sort();
        Self { points: copy }
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
    use rstest::rstest;
    use zequality::*;

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
        assert_zeq!(result, expected)
    }
}
