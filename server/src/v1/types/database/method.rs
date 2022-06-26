use crate::v1::types::database::*;
use crate::v1::types::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Method {
    /// The steps of the recipe
    steps: Vec<Step>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Step {
    /// The title of the step. Max 80 chars.
    title: String,
    /// Each of the substeps in the step
    substeps: Vec<SubStep>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct SubStep {
    /// The content of the substep. Max 2000 chars.
    content: Formattable,
    /// An accompanying image to the substep
    image: Option<Url>,
    /// Any warning present with this substep
    warnings: Vec<Warning>,
    /// Any information present with this substep
    infos: Vec<Info>,
}
