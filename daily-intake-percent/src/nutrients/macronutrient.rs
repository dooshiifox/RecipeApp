use crate::nutrients::{Nutrient, NutrientType};

pub struct Macronutrient<'a> {
    calories: Nutrient<'a>,
    fat: Nutrient<'a>,
    saturated_fat: Nutrient<'a>,
    cholesterol: Nutrient<'a>,
    carbs: Nutrient<'a>,
    fiber: Nutrient<'a>,
    sugar: Nutrient<'a>,
    protein: Nutrient<'a>,
}

impl<'a> Default for Macronutrient<'a> {
    fn default() -> Self {
        Self {
            calories: Nutrient::new("Calories", &["calories", "calorie", "cal"], "", 2000.0),
            fat: Nutrient::new("Fat", &["fat", "fats", "f"], "g", 78.0),
            saturated_fat: Nutrient::new(
                "Saturated Fat",
                &[
                    "saturated fat",
                    "saturated fats",
                    "saturated",
                    "satfat",
                    "sfat",
                    "sf",
                ],
                "g",
                20.0,
            ),
            cholesterol: Nutrient::new(
                "Cholesterol",
                &["cholesterol", "cholesterols", "chols", "chol"],
                "mg",
                300.0,
            ),
            carbs: Nutrient::new(
                "Carbohydrates",
                &["carbs", "carb", "carbs", "carbohydrate", "carbohydrates"],
                "g",
                300.0,
            ),
            fiber: Nutrient::new("Fiber", &["fiber", "fibers", "fib"], "g", 28.0),
            sugar: Nutrient::new("Sugar", &["sugar", "sugars", "sug"], "g", 50.0),
            protein: Nutrient::new("Protein", &["protein", "proteins", "prot"], "g", 50.0),
        }
    }
}

impl<'a> NutrientType for Macronutrient<'a> {
    fn get_name(&self) -> &'static str {
        "Macronutrients"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![
            &self.calories,
            &self.fat,
            &self.saturated_fat,
            &self.cholesterol,
            &self.carbs,
            &self.fiber,
            &self.sugar,
            &self.protein,
        ]
    }

    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![
            &mut self.calories,
            &mut self.fat,
            &mut self.saturated_fat,
            &mut self.cholesterol,
            &mut self.carbs,
            &mut self.fiber,
            &mut self.sugar,
            &mut self.protein,
        ]
    }
}
