use crate::v1::types::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Warning {
    /// The title of the warning. Max 80 chars.
    title: String,
    /// The content of the warning. Max 500 chars.
    content: Formattable,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Info {
    /// The title of the info. Should be max 80 chars.
    title: String,
    /// The content of the info. Should be max 500 chars.
    content: Formattable,
}
