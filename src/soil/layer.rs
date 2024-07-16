use super::model::SoilModel;

#[derive(Debug)]
pub struct SoilLayer {
    pub thickness: f64,
    pub soil_model: Box<dyn SoilModel>,
}
impl SoilLayer {
    #[must_use]
    pub fn new(thickness: f64, soil_model: Box<dyn SoilModel>) -> Self {
        Self {
            thickness,
            soil_model,
        }
    }
}
