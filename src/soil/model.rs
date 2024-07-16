use std::fmt::Debug;
pub trait SoilModel {
    fn unit_weight(&self) -> f64;
    fn compute_strain(&self, p0: f64, pd: f64) -> f64;
    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64;
}
impl Debug for dyn SoilModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoilModel")
            .field("Unit weight", &self.unit_weight())
            .finish()
    }
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
        if pd < 0.001 {
            return self.M;
        }
        if (p0 + pd) < pc {
            self.M
        } else if p0 > pc {
            self.m * (p0 + pd / 2.0)
        } else {
            let d1 = pc - p0;
            let d2 = p0 + pd - pc;

            let w1 = d1 * self.M;
            let w2 = d2 * (self.m * (pc + d2 / 2.0));

            (w1 + w2) / (pd)
        }

        // M for NC clay
    }
}
