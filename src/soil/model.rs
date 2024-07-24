use std::fmt::Debug;
#[allow(clippy::module_name_repetitions)]
pub trait SoilModel {
    fn unit_weight(&self) -> f64;
    fn compute_strain(&self, p0: f64, pd: f64) -> f64 {
        pd / self.elastic_modulus(p0, pd)
    }
    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64;
}
impl Debug for dyn SoilModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoilModel")
            .field("Unit weight", &self.unit_weight())
            .finish()
    }
}
#[allow(non_snake_case)]
pub struct General {
    pub unit_weight: f64,
    pub p_ref: f64,
    pub m: f64,
    pub a: f64,
}

impl Default for General {
    fn default() -> Self {
        Self {
            unit_weight: 19.0,
            m: 20.0,
            a: 1.0,
            p_ref: 0.0,
        }
    }
}
impl SoilModel for General {
    fn unit_weight(&self) -> f64 {
        self.unit_weight
    }
    #[allow(clippy::cast_precision_loss)]
    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64 {
        const LOAD_STEPS: usize = 10;
        const SIGMA_REF: f64 = 100.0;
        let dp = pd / (LOAD_STEPS - 1) as f64;

        (0..LOAD_STEPS).fold(0.0, |acc, i| {
            let sigma_m = p0 + dp * (0.5 + i as f64);
            acc + self.m * SIGMA_REF * (sigma_m / SIGMA_REF).powf(1.0 - self.a)
        }) / (LOAD_STEPS as f64)
    }
}
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Clay {
    pub unit_weight: f64,
    pub over_consolidation_ratio: f64,
    pub M: f64,
    pub m: f64,
    pub p_ref: f64,
}

impl Default for Clay {
    fn default() -> Self {
        Self {
            unit_weight: 19.0,
            over_consolidation_ratio: 1.0,
            M: 5000.0,
            m: 20.0,
            p_ref: 0.0,
        }
    }
}
impl Clay {
    /// # TODO
    /// 1. Quality control this shit
    fn pc(&self, p_0: f64) -> f64 {
        self.over_consolidation_ratio * p_0
    }
    /// # TODO
    /// 1. Quality control this shit
    fn stiffness_overconsolidated(&self) -> f64 {
        self.M
    }
    /// # TODO
    /// 1. Quality control this shit
    fn stiffness_normalconsolidated(&self, p0: f64, pd: f64) -> f64 {
        self.m * (p0 + pd / 2.0 - self.p_ref)
    }
}

impl SoilModel for Clay {
    fn unit_weight(&self) -> f64 {
        self.unit_weight
    }

    /// # TODO
    /// 1. Quality control this shit
    fn elastic_modulus(&self, p0: f64, pd: f64) -> f64 {
        let pc = self.pc(p0);
        if pd < 0.001 {
            return self.M;
        }
        if (p0 + pd) < pc {
            // Pure overconsolidated behaviour
            self.stiffness_overconsolidated()
        } else if p0 > pc {
            // Pure normalconsolidated behaviour
            self.stiffness_normalconsolidated(p0, pd)
        } else {
            // Mixed behaviour -> return a weighted average of stiffnesses
            let w1 = pc - p0;
            let w2 = p0 + pd - pc;

            let a = w1 * self.stiffness_overconsolidated();
            let b = w2 * self.stiffness_normalconsolidated(p0, pd);

            (a + b) / (pd)
        }
    }
}
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn create_clay() {
        let clay = Clay::default();
        dbg!(clay);
    }

    #[test]
    fn over_consolidation_ratio() {
        let clay = Clay {
            over_consolidation_ratio: 1.2,
            ..Default::default()
        };
        assert_relative_eq!(clay.over_consolidation_ratio, clay.pc(1.0));
    }

    // #[test]
    // fn create_clay() {
    //     //drawdown= 20 kpa
    //     // z    | sigmatot  | sigmaeff  | delta
    //     // 8    | 152       | 82        | 16.3
    //     // 10   | 190       | 100       | 15.7
    //     // 12.5 | 237       | 122.5     | 16.2
    //     let clay = Clay::default();
    //     dbg!(clay);
    // }
}
