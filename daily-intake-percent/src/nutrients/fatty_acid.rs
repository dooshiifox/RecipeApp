use crate::nutrients::{Nutrient, NutrientType};

pub struct FattyAcid<'a> {
    omega_3s: Nutrient<'a>,
    omega_6s: Nutrient<'a>,
}

impl<'a> Default for FattyAcid<'a> {
    fn default() -> Self {
        Self {
            omega_3s: Nutrient::new("Omega 3s", &["3s", "o3s", "o3", "omega3"], "?", 0.0),
            omega_6s: Nutrient::new("Omega 6s", &["6s", "o6s", "o6", "omega6"], "?", 0.0),
        }
    }
}

impl<'a> NutrientType for FattyAcid<'a> {
    fn get_name(&self) -> &'static str {
        "Fatty Acids"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![&self.omega_3s, &self.omega_6s]
    }

    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![&mut self.omega_3s, &mut self.omega_6s]
    }
}
