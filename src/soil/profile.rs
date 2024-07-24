use crate::{delta, hydro::ProfilePorePressure, linspace, profile::Profile};

use super::layer::SoilLayer;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct SoilProfile {
    soil_layers: Vec<SoilLayer>,
    pore_pressure_profile: ProfilePorePressure,
    manual_bedrock_depth: Option<f64>,
}
impl SoilProfile {
    #[must_use]
    pub fn with_soil_layers(mut self, soil_layers: Vec<SoilLayer>) -> Self {
        self.soil_layers = soil_layers;
        self
    }
    #[must_use]
    pub fn with_pore_pressure_profile(
        mut self,
        pore_pressure_profile: ProfilePorePressure,
    ) -> Self {
        self.pore_pressure_profile = pore_pressure_profile;
        self
    }
    #[must_use]
    pub fn with_depth_to_bedrock(mut self, manual_bedrock_depth: f64) -> Self {
        self.manual_bedrock_depth = Some(manual_bedrock_depth);
        self
    }
    pub fn set_depth_to_bedrock(&mut self, manual_bedrock_depth: f64) {
        self.manual_bedrock_depth = Some(manual_bedrock_depth);
    }
}
impl SoilProfile {
    #[must_use]
    pub fn depth_to_bedrock(&self) -> f64 {
        match self.manual_bedrock_depth {
            Some(v) => v,
            None => self
                .soil_layers
                .iter()
                .fold(0.0, |acc, layer| acc + layer.thickness),
        }
    }
    #[must_use]
    pub fn in_situ_effective_stress(&self, depth: f64) -> Option<f64> {
        let total_stress_at_depth = self.in_situ_total_stress(depth);
        let pore_pressure_at_depth = self.pore_pressure_profile.eval(depth);

        total_stress_at_depth.map(|sigma| sigma - pore_pressure_at_depth)
    }
    #[must_use]
    pub fn pc(&self, depth: f64) -> Option<f64> {
        let _x = self.get_soil_layer(depth)?;
        todo!()
    }
    #[must_use]
    pub fn in_situ_total_stress(&self, depth: f64) -> Option<f64> {
        if depth < 0.0 {
            return None;
        }

        if depth > self.depth_to_bedrock() {
            None
        } else {
            let mut sum = 0.0;
            let mut z = 0.0;
            let mut layercount = 0;

            while z < depth {
                let soil_layer = &self.soil_layers[layercount];

                if z + soil_layer.thickness < depth {
                    z += soil_layer.thickness;
                    sum += soil_layer.thickness * soil_layer.soil_model.unit_weight();
                } else {
                    let dz = depth - z;
                    sum += dz * soil_layer.soil_model.unit_weight();
                    z = depth;
                }
                layercount += 1;
            }
            Some(sum)
        }
    }

    /// # Panics
    /// Assumed unreachable code reached
    #[must_use]
    pub fn get_soil_layer(&self, depth: f64) -> Option<&SoilLayer> {
        if depth < 0.0 {
            return None;
        }

        if depth > self.depth_to_bedrock() {
            None
        } else {
            let mut z = 0.0;
            let mut layercount = 0;

            while z <= depth {
                let soil_layer = &self.soil_layers[layercount];

                if z + soil_layer.thickness < depth {
                    z += soil_layer.thickness;
                    layercount += 1;
                } else {
                    return Some(soil_layer);
                }
            }
            // Should be unreachable code
            panic!("Unexpected");
        }
    }

    /// # Panics
    /// Idunno dude
    #[must_use]
    pub fn compute_settlement(&self, drawdown: &ProfilePorePressure) -> f64 {
        const n: usize = 100;
        let delta = delta(0.0, self.depth_to_bedrock(), n);

        linspace(0.0, self.depth_to_bedrock() - delta, n)
            .iter()
            .fold(0.0, |acc, &z| {
                let eval_depth = z + delta / 2.0;
                let p0 = self.in_situ_effective_stress(eval_depth).unwrap();
                let pd = self.pore_pressure_profile.eval(eval_depth) - drawdown.eval(eval_depth);

                let strain = self
                    .get_soil_layer(eval_depth)
                    .unwrap()
                    .soil_model
                    .compute_strain(p0, pd);
                acc + strain * delta
            })
    }
}
