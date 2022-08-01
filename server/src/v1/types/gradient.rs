/// Holds the gradient information for a Recipe.
///
/// Colors should match the regex `#[0-9a-f]{6}`.
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[readonly::make]
pub struct Gradient(String, String);
