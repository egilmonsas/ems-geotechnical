use std::fmt::Debug;
#[derive(Debug)]

pub struct SoilProfile {
    soil_layers: Vec<SoilLayer>,
}

impl SoilProfile {
    fn depth_to_bedrock(&self) -> f64 {
        self.soil_layers
            .iter()
            .fold(0.0, |acc, layer| acc + layer.thickness)
    }
    fn in_situ_total_stress(&self, depth: f64) -> Option<f64> {
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

                    if z + soil_layer.thickness < z {
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
}

#[derive(Debug)]
pub struct SoilLayer {
    thickness: f64,
    soil_model: Box<dyn SoilModel>,
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
}

pub struct Clay {
    unit_weight: f64,
    over_consolidation_ratio: f64,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use zequality::*;
    #[test]
    fn create_soil_layer() {
        let soil_layer = SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                unit_weight: 19.0,
                over_consolidation_ratio: 1.2,
            }),
        };

        dbg!(soil_layer);
    }
    #[test]
    fn create_soil_profile() {
        let soil_layer = SoilLayer {
            thickness: 1.0,
            soil_model: Box::new(Clay {
                unit_weight: 19.0,
                over_consolidation_ratio: 1.2,
            }),
        };
        let soil_layer2 = SoilLayer {
            thickness: 2.0,
            soil_model: Box::new(Clay {
                unit_weight: 19.0,
                over_consolidation_ratio: 1.2,
            }),
        };
        let soil_profile = SoilProfile {
            soil_layers: vec![soil_layer, soil_layer2],
        };
        dbg!(soil_profile);
    }
    #[rstest]
    #[case(20.0, 400.0)]
    #[case(10.0, 200.0)]
    #[case(30.0, 650.0)]
    #[case(5.0, 100.0)]

    fn in_situ_total_stress(#[case] eval_point: f64, #[case] expected: f64) {
        let soil_profile = SoilProfile {
            soil_layers: vec![
                SoilLayer {
                    thickness: 10.0,
                    soil_model: Box::new(Clay {
                        unit_weight: 20.0,
                        over_consolidation_ratio: 1.2,
                    }),
                },
                SoilLayer {
                    thickness: 10.0,
                    soil_model: Box::new(Clay {
                        unit_weight: 25.0,
                        over_consolidation_ratio: 1.2,
                    }),
                },
            ],
        };

        if let Some(result) = soil_profile.in_situ_total_stress(eval_point) {
            assert_zeq!(result, expected)
        }
    }

    #[rstest]
    #[case(45.0)]
    #[case(-5.0)]
    fn out_of_range_returns_none(#[case] eval_point: f64) {
        let soil_profile = SoilProfile {
            soil_layers: vec![
                SoilLayer {
                    thickness: 10.0,
                    soil_model: Box::new(Clay {
                        unit_weight: 20.0,
                        over_consolidation_ratio: 1.2,
                    }),
                },
                SoilLayer {
                    thickness: 10.0,
                    soil_model: Box::new(Clay {
                        unit_weight: 25.0,
                        over_consolidation_ratio: 1.2,
                    }),
                },
            ],
        };

        assert!(soil_profile.in_situ_total_stress(eval_point).is_none())
    }
}
