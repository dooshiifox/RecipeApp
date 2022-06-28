#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Formattable(String);

impl Formattable {
    /// Constructs a new Formattable with the content.
    pub fn new(content: impl ToString) -> Self {
        Self(content.to_string())
    }

    /// Constructs a new empty Formattable.
    pub fn new_empty() -> Self {
        Self::default()
    }
}

// Implements ToString on Formattable.
impl std::fmt::Display for Formattable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
