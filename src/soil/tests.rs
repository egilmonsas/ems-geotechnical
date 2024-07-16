use crate::hydro::ProfilePorePressure;
use crate::points::Point;

use super::{layer::*, model::*, profile::*};
use rstest::rstest;
#[test]
fn test_elastic_modulus() {
    let soil_layers = Clay {
        over_consolidation_ratio: 1.2,
        ..Default::default()
    };

    dbg!(soil_layers.elastic_modulus(100.0, 0.0));
    dbg!(soil_layers.elastic_modulus(100.0, 10.0));
    dbg!(soil_layers.elastic_modulus(100.0, 20.0));
    dbg!(soil_layers.elastic_modulus(100.0, 30.0));
    dbg!(soil_layers.elastic_modulus(100.0, 40.0));
}

#[test]
fn create_soil_layer() {
    let soil_layer = SoilLayer {
        thickness: 1.0,
        soil_model: Box::new(Clay::default()),
    };

    dbg!(soil_layer);
}
#[test]
fn create_soil_profile() {
    let soil_layer = SoilLayer {
        thickness: 1.0,
        soil_model: Box::new(Clay::default()),
    };
    let soil_layer2 = SoilLayer {
        thickness: 2.0,
        soil_model: Box::new(Clay::default()),
    };
    let soil_profile = SoilProfile::default().with_soil_layer(vec![soil_layer, soil_layer2]);
    dbg!(soil_profile);
}

#[rstest]
#[case(20.0, 380.0)]
#[case(10.0, 190.0)]
#[case(5.0, 95.0)]
fn in_situ_total_stress(#[case] eval_point: f64, #[case] expected: f64) {
    let soil_layers = vec![
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
    ];

    let soil_profile = SoilProfile::default().with_soil_layer(soil_layers);
    if let Some(result) = soil_profile.in_situ_total_stress(eval_point) {
        approx::assert_abs_diff_eq!(result, expected);
    }
}

#[rstest]
#[case(20.0, 230.0)]
#[case(10.0, 140.0)]
#[case(5.0, 95.0)]
fn in_situ_effective_stress(#[case] eval_point: f64, #[case] expected: f64) {
    let soil_layers = vec![
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
    ];
    let pore_pressure_profile =
        ProfilePorePressure::new(vec![Point::new(5.0, 0.0), Point::new(20.0, 150.0)]);

    let soil_profile = SoilProfile::default()
        .with_soil_layer(soil_layers)
        .with_pore_pressure_profile(pore_pressure_profile);

    if let Some(result) = soil_profile.in_situ_effective_stress(eval_point) {
        approx::assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn drawdown_settlement() {
    let soil_layers = vec![
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
    ];
    let pore_pressure_profile = ProfilePorePressure::new(vec![
        Point::new(5.0, 0.0),
        Point::new(15.0, 100.0),
        Point::new(20.0, 150.0),
    ]);

    let soil_profile = SoilProfile::default()
        .with_soil_layer(soil_layers)
        .with_pore_pressure_profile(pore_pressure_profile);

    let drawdown_profile = ProfilePorePressure::new(vec![
        Point::new(5.0, 0.0),
        Point::new(15.0, 100.0),
        Point::new(20.0, 125.0),
    ]);

    dbg!(soil_profile.compute_settlement(drawdown_profile));
}

#[rstest]
#[case(21.0)]
#[case(-5.0)]
fn out_of_range_returns_none(#[case] eval_point: f64) {
    let soil_layers = vec![
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
        SoilLayer {
            thickness: 10.0,
            soil_model: Box::new(Clay::default()),
        },
    ];

    let soil_profile = SoilProfile::default().with_soil_layer(soil_layers);

    assert!(soil_profile.in_situ_total_stress(eval_point).is_none());
}
