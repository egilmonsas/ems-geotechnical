use crate::profile::point::Point;

pub trait Profile {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(points: Vec<Point>) -> Self;

    fn points(&self) -> &Vec<Point>;

    fn point_below(&self, eval_point: f64) -> Option<&Point> {
        self.points()
            .iter()
            .filter(|&p| p.x <= eval_point)
            .collect::<Vec<&Point>>()
            .last()
            .copied()
    }

    fn point_above(&self, eval_point: f64) -> Option<&Point> {
        self.points()
            .iter()
            .filter(|&p| p.x >= eval_point)
            .collect::<Vec<&Point>>()
            .first()
            .copied()
    }

    fn point_around(&self, eval_point: f64) -> (Option<&Point>, Option<&Point>) {
        (self.point_below(eval_point), self.point_above(eval_point))
    }

    #[must_use]
    fn lerp(point_below: &Point, point_above: &Point, eval_point: f64) -> Point {
        if point_below == point_above {
            *point_below
        } else {
            let x0 = point_below.x;
            let x1 = point_above.x;

            point_above * ((eval_point - x0) / (x1 - x0))
                + (point_below * ((x1 - eval_point) / (x1 - x0)))
        }
    }

    // Lerp Evaluate between two known points
    fn eval(&self, eval_point: f64) -> f64 {
        let point_around = self.point_around(eval_point);

        match point_around {
            //case 1: x between two known points
            (Some(point_below), Some(point_above)) => {
                Self::lerp(point_below, point_above, eval_point).y
            }
            //case 2: x below range closest value or extrapolate? -> Not implemented yet
            (None, Some(point_above)) => point_above.y,
            //case 3: x above range closest value or extrapolate? -> Not implemented yet
            (Some(point_below), None) => point_below.y,
            (None, None) => panic!("No points found"),
        }
    }
}
