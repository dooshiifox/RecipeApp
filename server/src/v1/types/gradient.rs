/// Holds the gradient information for a Recipe.
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[readonly::make]
pub struct Gradient(String, String);
