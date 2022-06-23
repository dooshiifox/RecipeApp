use crate::nutrients::{Nutrient, NutrientType};

pub struct CarbsAndSugar<'a> {
    starch: Nutrient<'a>,
    sucrose: Nutrient<'a>,
    glucose: Nutrient<'a>,
    fructose: Nutrient<'a>,
    lactose: Nutrient<'a>,
    maltose: Nutrient<'a>,
    galactose: Nutrient<'a>,
}

impl<'a> Default for CarbsAndSugar<'a> {
    fn default() -> Self {
        Self {
            starch: Nutrient::new("Starch", &["starch"], "?", 0.0),
            sucrose: Nutrient::new("Sucrose", &["sucrose"], "?", 0.0),
            glucose: Nutrient::new("Glucose", &["glucose"], "?", 0.0),
            fructose: Nutrient::new("Fructose", &["fructose"], "?", 0.0),
            lactose: Nutrient::new("Lactose", &["lactose"], "?", 0.0),
            maltose: Nutrient::new("Maltose", &["maltose"], "?", 0.0),
            galactose: Nutrient::new("Galactose", &["galactose"], "?", 0.0),
        }
    }
}

impl<'a> NutrientType for CarbsAndSugar<'a> {
    fn get_name(&self) -> &'static str {
        "Carbs and Sugars"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![
            &self.starch,
            &self.sucrose,
            &self.glucose,
            &self.fructose,
            &self.lactose,
            &self.maltose,
            &self.galactose,
        ]
    }

    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![
            &mut self.starch,
            &mut self.sucrose,
            &mut self.glucose,
            &mut self.fructose,
            &mut self.lactose,
            &mut self.maltose,
            &mut self.galactose,
        ]
    }
}
