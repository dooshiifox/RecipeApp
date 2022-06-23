use crate::nutrients::{Nutrient, NutrientType};

pub struct Mineral<'a> {
    calcium: Nutrient<'a>,
    copper: Nutrient<'a>,
    iodine: Nutrient<'a>,
    iron: Nutrient<'a>,
    magnesium: Nutrient<'a>,
    manganese: Nutrient<'a>,
    phosphorus: Nutrient<'a>,
    potassium: Nutrient<'a>,
    selenium: Nutrient<'a>,
    sodium: Nutrient<'a>,
    zinc: Nutrient<'a>,
    ash: Nutrient<'a>,
    water: Nutrient<'a>,
}

impl<'a> Default for Mineral<'a> {
    fn default() -> Self {
        Self {
            calcium: Nutrient::new("Calcium", &["ca", "mca", "mc"], "mg", 1300.0),
            copper: Nutrient::new("Copper", &["cu", "mcu"], "mg", 0.9),
            iodine: Nutrient::new("Iodine", &["i", "iod", "miod", "mi"], "?", 0.0),
            iron: Nutrient::new("Iron", &["fe", "mfe", "mf"], "mg", 18.0),
            magnesium: Nutrient::new("Magnesium", &["mg", "mag", "mmg"], "mg", 420.0),
            manganese: Nutrient::new("Manganese", &["mn", "mman", "mmn"], "mg", 2.3),
            phosphorus: Nutrient::new("Phosphorus", &["p", "mp", "mph"], "mg", 1250.0),
            potassium: Nutrient::new("Potassium", &["k", "mk", "mpot"], "mg", 4700.0),
            selenium: Nutrient::new("Selenium", &["se", "mse", "msel"], "μg", 55.0),
            sodium: Nutrient::new("Sodium", &["na", "mna", "mna"], "mg", 2300.0),
            zinc: Nutrient::new("Zinc", &["zn", "mzn", "mz"], "mg", 11.0),
            ash: Nutrient::new("Ash", &["ash", "mash"], "?", 0.0),
            water: Nutrient::new("Water", &["mwater", "h20", "wat", "w"], "?", 0.0),
        }
    }
}

impl<'a> NutrientType for Mineral<'a> {
    fn get_name(&self) -> &'static str {
        "Minerals"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![
            &self.calcium,
            &self.copper,
            &self.iodine,
            &self.iron,
            &self.magnesium,
            &self.manganese,
            &self.phosphorus,
            &self.potassium,
            &self.selenium,
            &self.sodium,
            &self.zinc,
            &self.ash,
            &self.water,
        ]
    }

    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![
            &mut self.calcium,
            &mut self.copper,
            &mut self.iodine,
            &mut self.iron,
            &mut self.magnesium,
            &mut self.manganese,
            &mut self.phosphorus,
            &mut self.potassium,
            &mut self.selenium,
            &mut self.sodium,
            &mut self.zinc,
            &mut self.ash,
            &mut self.water,
        ]
    }
}
