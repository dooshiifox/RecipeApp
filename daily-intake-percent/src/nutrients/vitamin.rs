use crate::nutrients::{Nutrient, NutrientType};

pub struct Vitamin<'a> {
    a: Nutrient<'a>,
    b1: Nutrient<'a>,
    b2: Nutrient<'a>,
    b3: Nutrient<'a>,
    b5: Nutrient<'a>,
    b6: Nutrient<'a>,
    b9: Nutrient<'a>,
    b12: Nutrient<'a>,
    c: Nutrient<'a>,
    d: Nutrient<'a>,
    e: Nutrient<'a>,
    k: Nutrient<'a>,
    choline: Nutrient<'a>,
    lycopene: Nutrient<'a>,
    lutein_zeazanthin: Nutrient<'a>,
}

impl<'a> Default for Vitamin<'a> {
    fn default() -> Self {
        Self {
            a: Nutrient::new("Vitamin A", &["a", "vita", "vit a", "va"], "IU", 3000.0),
            b1: Nutrient::new("Vitamin B1", &["b1", "vitb1", "vit b1", "vb1"], "mg", 1.2),
            b2: Nutrient::new("Vitamin B2", &["b2", "vitb2", "vit b2", "vb2"], "mg", 1.3),
            b3: Nutrient::new("Vitamin B3", &["b3", "vitb3", "vit b3", "vb3"], "mg", 16.0),
            b5: Nutrient::new("Vitamin B5", &["b5", "vitb5", "vit b5", "vb5"], "mg", 5.0),
            b6: Nutrient::new("Vitamin B6", &["b6", "vitb6", "vit b6", "vb6"], "mg", 1.7),
            b9: Nutrient::new("Vitamin B9", &["b9", "vitb9", "vit b9", "vb9"], "μg", 400.0),
            b12: Nutrient::new(
                "Vitamin B12",
                &["b12", "vitb12", "vit b12", "vb12"],
                "μg",
                2.4,
            ),
            c: Nutrient::new("Vitamin C", &["c", "vit c", "vit c", "vc"], "mg", 90.0),
            d: Nutrient::new("Vitamin D", &["d", "vit d", "vit d", "vd"], "μg", 20.0),
            e: Nutrient::new("Vitamin E", &["e", "vit e", "vit e", "ve"], "mg", 15.0),
            k: Nutrient::new("Vitamin K", &["k", "vit k", "vit k", "vk"], "mg", 120.0),
            choline: Nutrient::new(
                "Choline",
                &["vitamin choline", "vitcholine", "choli", "chl", "vch"],
                "mg",
                550.0,
            ),
            lycopene: Nutrient::new(
                "Lycopene",
                &["vitamin lycopene", "vitlycopene", "lyco", "lyc", "vl"],
                "?",
                0.0,
            ),
            lutein_zeazanthin: Nutrient::new(
                "Lutein & Zeazanthin",
                &[
                    "lutein",
                    "zeazanthin",
                    "lz",
                    "vlz",
                    "vit lutein",
                    "vit zeazanthin",
                    "vit lz",
                    "vit zz",
                    "vit l",
                    "vit z",
                ],
                "?",
                0.0,
            ),
        }
    }
}

impl<'a> NutrientType for Vitamin<'a> {
    fn get_name(&self) -> &'static str {
        "Vitamins"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![
            &self.a,
            &self.b1,
            &self.b2,
            &self.b3,
            &self.b5,
            &self.b6,
            &self.b9,
            &self.b12,
            &self.c,
            &self.d,
            &self.e,
            &self.k,
            &self.choline,
            &self.lycopene,
            &self.lutein_zeazanthin,
        ]
    }
    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![
            &mut self.a,
            &mut self.b1,
            &mut self.b2,
            &mut self.b3,
            &mut self.b5,
            &mut self.b6,
            &mut self.b9,
            &mut self.b12,
            &mut self.c,
            &mut self.d,
            &mut self.e,
            &mut self.k,
            &mut self.choline,
            &mut self.lycopene,
            &mut self.lutein_zeazanthin,
        ]
    }
}
