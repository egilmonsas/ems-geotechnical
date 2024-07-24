use ems_geotechnical::{
    delta,
    hydro::ProfilePorePressure,
    linspace,
    profile::Point,
    soil::{layer::SoilLayer, model::Clay, profile::SoilProfile},
};
use plotters::prelude::*;

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
                over_consolidation_ratio: 1.175,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 2.5,
            soil_model: Box::new(Clay {
                M: 10000.0,
                over_consolidation_ratio: 1.15,
                ..Default::default()
            }),
        },
        SoilLayer {
            thickness: 5.0,
            soil_model: Box::new(Clay {
                M: 12500.0,
                over_consolidation_ratio: 1.125,
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
    let mut soil_profile = SoilProfile::default()
        .with_soil_layers(soil_layers)
        .with_pore_pressure_profile(pore_pressure_profile);
    let precision = 20;
    // println!("     dybde |    sigma_tot    sigma_eff");
    // println!("______________________________________");
    // for i in 0..=precision * soil_profile.depth_to_bedrock() as usize {
    //     let depth = (i as f64) / precision as f64;

    //     println!(
    //         "{depth:>width$.precision$} m | {sig_tot:>width$.precision$} kPa {sig_eff:>width$.precision$} kPa",
    //         width = 8,
    //         precision = 3,
    //         sig_tot = soil_profile.in_situ_total_stress(depth).unwrap(),
    //         sig_eff = soil_profile.in_situ_effective_stress(depth).unwrap()
    //     );
    // }
    let root_area = SVGBackend::new("image.svg", (2000, 2000)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Setninger", ("sans-serif", 40))
        .build_cartesian_2d(0.0..30.0, -0.0..100.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    for drawdown in linspace(-10.0, -50.0, 5) {
        ctx.draw_series(LineSeries::new(
            linspace(0.0, 20.0, 200).iter().map(|&x| {
                let profile = ProfilePorePressure::drawdown_profile(
                    &ProfilePorePressure::new(vec![
                        Point::new(0.0, 0.0),
                        Point::new(1.0, 0.0),
                        Point::new(x, (x - 1.0) * 10.0),
                    ]),
                    drawdown,
                );
                let mut y = 0.0;
                soil_profile.set_depth_to_bedrock(x + 0.2);
                y += soil_profile.compute_settlement(&profile);
                soil_profile.set_depth_to_bedrock(x + 0.1);
                y += 2.0 * soil_profile.compute_settlement(&profile);
                soil_profile.set_depth_to_bedrock(x);
                y += 4.0 * soil_profile.compute_settlement(&profile);
                soil_profile.set_depth_to_bedrock((x - 0.1).max(0.0));
                y += 2.0 * soil_profile.compute_settlement(&profile);
                soil_profile.set_depth_to_bedrock((x - 0.2).max(0.0));
                y += soil_profile.compute_settlement(&profile);

                (x, (y / 10.0) * 1000.0)
            }),
            &GREEN,
        ))
        .unwrap();

        ctx.draw_series(LineSeries::new(
            linspace(0.0, 20.0, 200).iter().map(|&x| {
                let profile = ProfilePorePressure::drawdown_profile(
                    &ProfilePorePressure::new(vec![
                        Point::new(0.0, 0.0),
                        Point::new(1.0, 0.0),
                        Point::new(x, (x - 1.0) * 10.0),
                    ]),
                    drawdown,
                );
                let delta = delta(0.0, 30.0, 3000);
                let mut y = 0.0;
                soil_profile.set_depth_to_bedrock(x);

                y += soil_profile.compute_settlement(&profile);

                (x, (y / 1.0) * 1000.0)
            }),
            &RED,
        ))
        .unwrap();
    }
}
