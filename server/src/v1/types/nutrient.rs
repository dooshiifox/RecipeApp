/// A type representing a specific nutrient.
///
/// Internally is a `u16` as this allows for more efficient storage,
/// however has several functions that make it easier to use as a `String`.
#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Nutrient(u16);
