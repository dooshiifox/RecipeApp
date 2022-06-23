use crate::nutrients::{Nutrient, NutrientType};

pub struct AminoAcid<'a> {
    tryptophan: Nutrient<'a>,
    histidine: Nutrient<'a>,
    threonine: Nutrient<'a>,
    isoleucine: Nutrient<'a>,
    lysine: Nutrient<'a>,
    leucine: Nutrient<'a>,
    methionine: Nutrient<'a>,
    cystine: Nutrient<'a>,
    phenylalanine: Nutrient<'a>,
    tyrosine: Nutrient<'a>,
    valine: Nutrient<'a>,
    arginine: Nutrient<'a>,
    alanine: Nutrient<'a>,
    aspartic_acid: Nutrient<'a>,
    glutamic_acid: Nutrient<'a>,
    glycine: Nutrient<'a>,
    proline: Nutrient<'a>,
    serine: Nutrient<'a>,
    hydroxyproline: Nutrient<'a>,
}

impl<'a> Default for AminoAcid<'a> {
    fn default() -> Self {
        Self {
            tryptophan: Nutrient::new("Tryptophan", &[], "?", 0.0),
            histidine: Nutrient::new("Histidine", &[], "?", 0.0),
            threonine: Nutrient::new("Threonine", &[], "?", 0.0),
            isoleucine: Nutrient::new("Isoleucine", &[], "?", 0.0),
            lysine: Nutrient::new("Lysine", &[], "?", 0.0),
            leucine: Nutrient::new("Leucine", &[], "?", 0.0),
            methionine: Nutrient::new("Methionine", &[], "?", 0.0),
            cystine: Nutrient::new("Cystine", &[], "?", 0.0),
            phenylalanine: Nutrient::new("Phenylalanine", &[], "?", 0.0),
            tyrosine: Nutrient::new("Tyrosine", &[], "?", 0.0),
            valine: Nutrient::new("Valine", &[], "?", 0.0),
            arginine: Nutrient::new("Arginine", &[], "?", 0.0),
            alanine: Nutrient::new("Alanine", &[], "?", 0.0),
            aspartic_acid: Nutrient::new("Aspartic Acid", &[], "?", 0.0),
            glutamic_acid: Nutrient::new("Glutamic Acid", &[], "?", 0.0),
            glycine: Nutrient::new("Glycine", &[], "?", 0.0),
            proline: Nutrient::new("Proline", &[], "?", 0.0),
            serine: Nutrient::new("Serine", &[], "?", 0.0),
            hydroxyproline: Nutrient::new("Hydroxyproline", &[], "?", 0.0),
        }
    }
}

impl<'a> NutrientType for AminoAcid<'a> {
    fn get_name(&self) -> &'static str {
        "Amino Acids"
    }

    fn get_nutrients(&self) -> Vec<&Nutrient> {
        vec![
            &self.tryptophan,
            &self.histidine,
            &self.threonine,
            &self.isoleucine,
            &self.lysine,
            &self.leucine,
            &self.methionine,
            &self.cystine,
            &self.phenylalanine,
            &self.tyrosine,
            &self.valine,
            &self.arginine,
            &self.alanine,
            &self.aspartic_acid,
            &self.glutamic_acid,
            &self.glycine,
            &self.proline,
            &self.serine,
            &self.hydroxyproline,
        ]
    }

    fn get_nutrients_mut(&mut self) -> Vec<&'a mut Nutrient> {
        vec![
            &mut self.tryptophan,
            &mut self.histidine,
            &mut self.threonine,
            &mut self.isoleucine,
            &mut self.lysine,
            &mut self.leucine,
            &mut self.methionine,
            &mut self.cystine,
            &mut self.phenylalanine,
            &mut self.tyrosine,
            &mut self.valine,
            &mut self.arginine,
            &mut self.alanine,
            &mut self.aspartic_acid,
            &mut self.glutamic_acid,
            &mut self.glycine,
            &mut self.proline,
            &mut self.serine,
            &mut self.hydroxyproline,
        ]
    }
}
