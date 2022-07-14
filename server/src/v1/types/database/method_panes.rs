use crate::v1::types::*;

/// A Warning pane used in the [`SubStep`] of a [`Method`].
///
/// [`SubStep`]: crate::v1::types::database::SubStep
/// [`Method`]: crate::v1::types::database::Method
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Warning {
    /// The title of the warning.
    title: String,
    /// The content of the warning.
    content: Formattable,
}

impl Warning {
    /// Constructs a new Warning.
    pub fn new(title: String, content: impl ToString) -> Self {
        Self {
            title,
            content: Formattable::new(content),
        }
    }

    /// Creates a new empty Warning.
    pub fn new_empty() -> Self {
        Self::default()
    }

    /// Sets the title on the Warning.
    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the content on the Warning.
    pub fn set_content(mut self, content: impl ToString) -> Self {
        self.content = Formattable::new(content);
        self
    }
}

/// An Information pane used in the [`SubStep`] of a [`Method`].
///
/// [`SubStep`]: crate::v1::types::database::SubStep
/// [`Method`]: crate::v1::types::database::Method
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the info.
    title: String,
    /// The content of the info.
    content: Formattable,
}

impl Info {
    /// Constructs a new Info.
    pub fn new(title: String, content: Formattable) -> Self {
        Self { title, content }
    }

    /// Creates a new empty Info.
    pub fn new_empty() -> Self {
        Self::default()
    }

    /// Sets the title on the Info.
    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the content on the Info.
    pub fn set_content(mut self, content: impl ToString) -> Self {
        self.content = Formattable::new(content.to_string());
        self
    }
}
