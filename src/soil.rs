use std::fmt::Debug;

use crate::{profile::Profile, ProfilePorePressure};

#[derive(Debug, Default)]

pub struct SoilProfile {
    soil_layers: Vec<SoilLayer>,
    pore_pressure_profile: ProfilePorePressure,
}
impl SoilProfile {
    pub fn with_soil_layer(mut self, soil_layers: Vec<SoilLayer>) -> Self {
        self.soil_layers = soil_layers;
        self
    }
    pub fn with_pore_pressure_profile(
        mut self,
        pore_pressure_profile: ProfilePorePressure,
    ) -> Self {
        self.pore_pressure_profile = pore_pressure_profile;
        self
    }
}

impl SoilProfile {
    pub fn depth_to_bedrock(&self) -> f64 {
        self.soil_layers
            .iter()
            .fold(0.0, |acc, layer| acc + layer.thickness)
    }
    pub fn in_situ_effective_stress(&self, depth: f64) -> Option<f64> {
        let total_stress_at_depth = self.in_situ_total_stress(depth);
        let pore_pressure_at_depth = self.pore_pressure_profile.eval(depth);

        total_stress_at_depth.map(|sigma| sigma - pore_pressure_at_depth)
    }
    pub fn pc(&self, depth: f64) -> Option<f64> {
        let x = self.get_soil_layer(depth)?;
        todo!()
    }
    pub fn in_situ_total_stress(&self, depth: f64) -> Option<f64> {
        if depth < 0.0 {
            return None;
        }

        match depth > self.depth_to_bedrock() {
            true => None,
            false => {
                let mut sum = 0.0;
                let mut z = 0.0;
                let mut layercount = 0;

                while z < depth {
                    let soil_layer = &self.soil_layers[layercount];

                    if z + soil_layer.thickness < depth {
                        z += soil_layer.thickness;
                        sum += soil_layer.thickness * soil_layer.soil_model.unit_weight()
                    } else {
                        let dz = depth - z;
                        sum += dz * soil_layer.soil_model.unit_weight();
                        z = depth
                    }
                    layercount += 1
                }
                Some(sum)
            }
        }
    }
    pub fn get_soil_layer(&self, depth: f64) -> Option<&SoilLayer> {
        if depth < 0.0 {
            return None;
        }

        match depth > self.depth_to_bedrock() {
            true => None,
            false => {
                let mut z = 0.0;
                let mut layercount = 0;

                while z <= depth {
                    let soil_layer = &self.soil_layers[layercount];

                    if z + soil_layer.thickness < depth {
                        z += soil_layer.thickness;
                        layercount += 1
                    } else {
                        return Some(soil_layer);
                    }
                }
                panic!("boi");
            }
        }
    }

    pub fn compute_settlement(&self, drawdown: ProfilePorePressure) -> f64 {
        const DZ: f64 = 0.5;
        let mut z = 0.0;
        let mut settlement = 0.0;
        while z < self.depth_to_bedrock() {
            let eval_depth = z + DZ / 2.0;
            let p0 = self.in_situ_effective_stress(z).unwrap();
            let pd = self.pore_pressure_profile.eval(eval_depth) - drawdown.eval(eval_depth);

            let strain = self
                .get_soil_layer(z)
                .unwrap()
                .soil_model
                .compute_strain(p0, pd);

            let d_epsilon = strain * DZ;
            settlement += d_epsilon;
            z += DZ;
        }
        settlement
    }
}

#[derive(Debug)]
pub struct SoilLayer {
    thickness: f64,
    soil_model: Box<dyn SoilModel>,
}
impl SoilLayer {
    pub fn new(thickness: f64, soil_model: Box<dyn SoilModel>) -> Self {
        Self {
            thickness,
            soil_model,
        }
    }
}
impl Debug for dyn SoilModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoilModel")
            .field("Unit weight", &self.unit_weight())
            .finish()
    }
}
pub trait SoilModel {
    fn unit_weight(&self) -> f64;
    fn compute_strain(&self, p0: f64, pd: f64) -> f64;
    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64;
}

pub struct Clay {
    pub unit_weight: f64,
    pub over_consolidation_ratio: f64,
    pub M: f64,
    pub m: f64,
}

impl Default for Clay {
    fn default() -> Self {
        Self {
            unit_weight: 19.0,
            over_consolidation_ratio: 1.0,
            M: 5000.0,
            m: 20.0,
        }
    }
}
impl Clay {
    fn pc(&self, sigma_0: f64) -> f64 {
        self.over_consolidation_ratio * sigma_0
    }
}

impl SoilModel for Clay {
    fn unit_weight(&self) -> f64 {
        self.unit_weight
    }

    fn compute_strain(&self, p0: f64, pd: f64) -> f64 {
        pd / self.elastic_modulus(p0, pd)
    }

    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64 {
        let pc = self.pc(p0);
        if p0 + pd < pc {
            self.M
        } else if p0 > pc {
            self.m * (p0 + pd / 2.0)
        } else {
            ((pc - p0) * self.M + self.m * (pc + (pd - pc - p0) / 2.0) * (p0 + pd - pc)) / (pd - p0)
        }

        // M for NC clay
    }
}

#[cfg(test)]
mod tests {

    use crate::Point;

    use super::*;
    use rstest::rstest;
    use zequality::*;
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
            assert_zeq!(result, expected)
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
            assert_zeq!(result, expected)
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

        assert!(soil_profile.in_situ_total_stress(eval_point).is_none())
    }
}
