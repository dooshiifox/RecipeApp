use serde::de::Error;
use std::collections::HashMap;

lazy_static::lazy_static! {
    /// The list of nutrients found, along with their IDs.
    ///
    /// # IMPORTANT IF YOU ARE GOING TO CHANGE THIS!
    ///
    /// This was originally a Vec of strings, but was changed to a HashMap to
    /// discourage people from editing the list.
    ///
    /// If you are going to insert something, CHECK THE ID IS NOT TAKEN FIRST
    /// AND HAS NEVER BEEN TAKEN IN THE PAST.
    ///
    /// If you are going to remove something, DO NOT!
    ///
    /// Modifying this stuff will change how the database works. Something
    /// with an ID of 3 in the database CURRENTLY and should ALWAYS refer to
    /// Saturated Fat.
    pub static ref NUTRIENTS: HashMap<u16, &'static str> = HashMap::from(
        [
            (1, "Calories"),
            (2, "Fat"),
            (3, "Saturated Fat"),
            (4, "Cholesterol"),
            (5, "Carbohydrates"),
            (6, "Fiber"),
            (7, "Sugar"),
            (8, "Protein"),
            (9, "Vitamin A"),
            (10, "Vitamin B1"),
            (11, "Vitamin B2"),
            (12, "Vitamin B3"),
            (13, "Vitamin B5"),
            (14, "Vitamin B6"),
            (15, "Vitamin B9"),
            (16, "Vitamin B12"),
            (17, "Vitamin C"),
            (18, "Vitamin D"),
            (19, "Vitamin E"),
            (20, "Vitamin K"),
            (21, "Choline"),
            (22, "Lycopene"),
            (23, "Lutein & Zeazanthin"),
            (24, "Calcium"),
            (25, "Copper"),
            (26, "Iodine"),
            (27, "Iron"),
            (28, "Magnesium"),
            (29, "Manganese"),
            (30, "Phosphorus"),
            (31, "Potassium"),
            (32, "Selenium"),
            (33, "Sodium"),
            (34, "Zinc"),
            (35, "Ash"),
            (36, "Water"),
            (37, "Starch"),
            (38, "Sucrose"),
            (39, "Glucose"),
            (40, "Fructose"),
            (41, "Lactose"),
            (42, "Maltose"),
            (43, "Galactose"),
            (44, "Omega 3S"),
            (45, "Omega 6S"),
            (46, "Tryptophan"),
            (47, "Histidine"),
            (48, "Threonine"),
            (49, "Isoleucine"),
            (50, "Lysine"),
            (51, "Leucine"),
            (52, "Methionine"),
            (53, "Cystine"),
            (54, "Phenylalanine"),
            (55, "Tyrosine"),
            (56, "Valine"),
            (57, "Arginine"),
            (58, "Alanine"),
            (59, "Aspartic Acid"),
            (60, "Glutamic Acid"),
            (61, "Glycine"),
            (62, "Proline"),
            (63, "Serine"),
            (64, "Hydroxyproline"),
        ]
    );
}

/// A type representing a specific nutrient.
///
/// Internally is a `u16` as this allows for more efficient storage,
/// however has several functions that make it easier to use as a `String`.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Nutrient(u16);

impl Nutrient {
    /// Returns a `Nutrient` from a str.
    pub fn from_str(s: &str) -> Option<Nutrient> {
        let s = s.to_lowercase();
        NUTRIENTS
            .iter()
            .find(|(_, v)| v.to_lowercase() == s)
            .map(|(k, _)| Nutrient(*k))
    }

    /// Returns an `&str` from a `Nutrient`.
    ///
    /// Panics if the `Nutrient` ID is invalid.
    pub fn as_str(&self) -> &'static str {
        NUTRIENTS[&self.0]
    }
}

impl std::fmt::Display for Nutrient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            NUTRIENTS.get(&self.0).unwrap_or(&"Invalid Nutrient")
        )
    }
}

impl From<Nutrient> for u16 {
    fn from(nutrient: Nutrient) -> u16 {
        nutrient.0
    }
}

impl From<u16> for Nutrient {
    fn from(nutrient: u16) -> Nutrient {
        Nutrient(nutrient)
    }
}

impl From<Nutrient> for String {
    fn from(nutrient: Nutrient) -> String {
        nutrient.to_string()
    }
}

impl From<Nutrient> for &str {
    fn from(nutrient: Nutrient) -> &'static str {
        nutrient.as_str()
    }
}

impl From<Nutrient> for mongodb::bson::Bson {
    fn from(nutrient: Nutrient) -> mongodb::bson::Bson {
        mongodb::bson::Bson::Int32(nutrient.0 as i32)
    }
}

impl From<mongodb::bson::Bson> for Nutrient {
    fn from(bson: mongodb::bson::Bson) -> Nutrient {
        match bson {
            mongodb::bson::Bson::Int32(i) => Nutrient(i as u16),
            _ => panic!("Invalid BSON type for Nutrient"),
        }
    }
}

/// A struct wrapping around `Nutrient` that can serialize and
/// deserialize to and from a `String`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SerdeStringNutrient(Nutrient);

impl SerdeStringNutrient {
    /// Creates a new `SerdeStringNutrient` from a `String`.
    pub fn from_string(s: String) -> Option<Self> {
        Nutrient::from_str(&s).map(Self)
    }

    /// Converts it to a `Nutrient`.
    pub fn into_nutrient(self) -> Nutrient {
        self.0
    }
}

impl serde::Serialize for SerdeStringNutrient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SerdeStringNutrient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Nutrient::from_str(&s)
            .map(Self)
            .ok_or_else(|| D::Error::custom(format!("Invalid nutrient: {}", s)))
    }
}

impl From<Nutrient> for SerdeStringNutrient {
    fn from(nutrient: Nutrient) -> Self {
        Self(nutrient)
    }
}

impl From<SerdeStringNutrient> for Nutrient {
    fn from(nutrient: SerdeStringNutrient) -> Self {
        nutrient.0
    }
}

impl std::ops::Deref for SerdeStringNutrient {
    type Target = Nutrient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
