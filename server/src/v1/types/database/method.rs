use crate::v1::types::database::*;
use crate::v1::types::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Method {
    /// The steps of the recipe
    steps: Vec<Step>,
}

impl Method {
    /// Contructs a new empty method.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new [`Step`] to the method.
    ///
    /// [`Step`]: crate::v1::types::database::Step
    pub fn add_step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Step {
    /// The title of the step.
    title: String,
    /// Each of the substeps in the step
    substeps: Vec<SubStep>,
}

impl Step {
    /// Constructs a new step with a title.
    pub fn new(title: String) -> Self {
        Self {
            title,
            substeps: vec![],
        }
    }

    /// Constructs a new step with a title and substeps
    pub fn new_with_substeps(title: String, substeps: Vec<SubStep>) -> Self {
        Self { title, substeps }
    }

    /// Adds a new substep to the step
    pub fn add_substep(mut self, substep: SubStep) -> Self {
        self.substeps.push(substep);
        self
    }
}

/// A SubStep is part of a larger [`Step`], which is part of the collection
/// of steps called a [`Method`]. This holds information about a single
/// paragraph of information, as well as an additional image and any
/// information about the nutrients/food or warnings about how to make it
/// safely.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct SubStep {
    /// The content of the substep.
    content: Formattable,
    /// An accompanying image to the substep
    image: Option<Url>,
    /// Any warning present with this substep
    warnings: Vec<Warning>,
    /// Any information present with this substep
    infos: Vec<Info>,
}

impl SubStep {
    /// Constructs a Builder for a Substep.
    pub fn builder() -> SubStepBuilder {
        SubStepBuilder::default()
    }
}

/// A SubStepBuilder helps in constructing a [`SubStep`] by using the
/// Builder pattern.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct SubStepBuilder {
    /// The content of the substep. Max 2000 chars.
    content: Option<Formattable>,
    /// An accompanying image to the substep
    image: Option<Url>,
    /// Any warning present with this substep
    warnings: Vec<Warning>,
    /// Any information present with this substep
    infos: Vec<Info>,
}

impl SubStepBuilder {
    /// Construct the [`SubStep`] from the SubStepBuilder.
    ///
    /// Errors if the content field is not set.
    pub fn build(self) -> Result<SubStep, String> {
        Ok(SubStep {
            content: self
                .content
                .ok_or_else(|| "No content set on substep".to_string())?,
            image: self.image,
            warnings: self.warnings,
            infos: self.infos,
        })
    }

    /// Sets the content on the SubStep.
    pub fn content(mut self, content: Formattable) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets the image on the SubStep.
    pub fn image(mut self, image: Url) -> Self {
        self.image = Some(image);
        self
    }

    /// Adds a Warning pane to the SubStep.
    pub fn add_warning(mut self, warning: Warning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Adds an Information pane to the SubStep.
    pub fn add_info(mut self, info: Info) -> Self {
        self.infos.push(info);
        self
    }
}
