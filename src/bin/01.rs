use ems_geotechnical::{
    hydro::ProfilePorePressure,
    profile::Point,
    soil::{layer::SoilLayer, model::Clay, profile::SoilProfile},
};

fn main() {
    let soil_layers = vec![
        SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                M: 7500.0,
                over_consolidation_ratio: 10.0,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                M: 7500.0,
                over_consolidation_ratio: 5.0,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                M: 7500.0,
                over_consolidation_ratio: 4.0,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                M: 5000.0,
                over_consolidation_ratio: 3.0,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                M: 5000.0,
                over_consolidation_ratio: 1.5,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 2.5,
            soil_model: Box::new(Clay {
                M: 6000.0,
                over_consolidation_ratio: 1.2,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 2.5,
            soil_model: Box::new(Clay {
                M: 7000.0,
                over_consolidation_ratio: 1.2,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 2.5,
            soil_model: Box::new(Clay {
                M: 8000.0,
                over_consolidation_ratio: 1.1,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 2.5,
            soil_model: Box::new(Clay {
                M: 10000.0,
                over_consolidation_ratio: 1.1,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 5.0,
            soil_model: Box::new(Clay {
                M: 12500.0,
                over_consolidation_ratio: 1.1,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 5.0,
            soil_model: Box::new(Clay {
                M: 15000.0,
                over_consolidation_ratio: 1.1,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 5.0,
            soil_model: Box::new(Clay {
                M: 20000.0,
                over_consolidation_ratio: 1.1,
                ..Default::default()
            }),
        },
    ];
    let pore_pressure_profile = ProfilePorePressure::new(vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(30.0, 290.0),
    ]);
    let soil_profile = SoilProfile::default()
        .with_soil_layers(soil_layers)
        .with_pore_pressure_profile(pore_pressure_profile);
    let precision = 20;
    println!("     dybde |    sigma_tot    sigma_eff");
    println!("______________________________________");
    for i in 0..=precision * soil_profile.depth_to_bedrock() as usize {
        let depth = (i as f64) / precision as f64;

        println!(
            "{depth:>width$.precision$} m | {sig_tot:>width$.precision$} kPa {sig_eff:>width$.precision$} kPa",
            width = 8,
            precision = 3,
            sig_tot = soil_profile.in_situ_total_stress(depth).unwrap(),
            sig_eff = soil_profile.in_situ_effective_stress(depth).unwrap()
        );
    }

    // for depth_to_bedrock in [
    //     0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 12.5, 15.0, 17.5, 20.0, 25.0, 30.0,
    // ] {
    //     for drawdown in [00.0, -10.0, -20.0, -30.0, -40.0, -50.0] {
    //         let profile = ProfilePorePressure::drawdown_profile(
    //             &ProfilePorePressure::new(vec![
    //                 Point::new(0.0, 0.0),
    //                 Point::new(1.0, 0.0),
    //                 Point::new(depth_to_bedrock, (depth_to_bedrock - 1.0) * 10.0),
    //             ]),
    //             drawdown,
    //         );
    //         soil_profile.set_depth_to_bedrock(depth_to_bedrock);
    //         println!(
    //             "{depth_to_bedrock:>width$.precision$} {drawdown:>width$.precision$} {delta_z:>width$.precision$}",
    //             width = 8,
    //             precision = 3,
    //             delta_z = soil_profile.compute_settlement(&profile) * 1000.0
    //         );
    //     }
    // }
}
