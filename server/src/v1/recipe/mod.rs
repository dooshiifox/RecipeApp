use crate::v1::types::*;

pub mod get;
pub mod get_basic;
pub mod post;
pub mod weekly;

/// The type of recipe sent in the request body.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRecipe {
    /// The unique identifier of the recipe.
    /// If set, overwrites the recipe with the same id.
    uuid: Option<Uuid>,
    /// The date the recipe was added to the database.
    /// If not set, defaults to the time the request was made.
    date_added: Option<Date>,
    /// The date the recipe will/went public. If not yet that date, can only be referred to by id.
    /// If not set, defaults to the time the request was made.
    becomes_public: Option<Date>,
    /// The staff who helped make this recipe.
    authors: Option<Vec<Uuid>>,
    /// A short string crediting the creators of the recipe. Max 400 chars.
    credits: Option<Formattable>,
    /// The date the recipe went weekly. None if never was set_log_levelweekly.
    weekly_timestamp: Option<Date>,
    /// The title of the recipe. Max 80 chars.
    title: Option<String>,
    /// A list of common nutrients found in the recipe. Should be 1-3 long
    nutrients: Option<Vec<SerdeStringNutrient>>,
    /// The time to cook the recipe, in minutes
    time_to_cook: Option<u16>,
    /// The servings of the recipe.
    servings: Option<u16>,
    /// The URL to the recipe image. Should be on S3
    image: Option<Url>,
    /// The ingredients of the recipe. Max 80chars per ingredient.
    ingredients: Option<Vec<String>>,
    /// The recipe's method
    method: Option<database::Method>,
    /// The quiz information for the end of the recipe
    quiz: Option<database::Quiz>,
}

impl RequestRecipe {
    /// Tries to convert a RequestRecipe into a [`Recipe`].
    pub fn into_recipe(self) -> Result<database::Recipe, String> {
        let mut builder = database::Recipe::builder();

        /// Expands to ```if let Some(value) = self.[field] {
        ///     builder.[setter](value);
        /// }```
        macro_rules! if_some(
            ($field:tt, $setter:ident) => {
                if let Some(value) = self.$field {
                    builder = builder.$setter(value);
                }
            };
        );

        // Set all of the optionable fields on the recipe.
        if_some!(uuid, uuid);
        if_some!(date_added, date_added);
        if_some!(becomes_public, becomes_public);
        if_some!(authors, set_authors);
        if_some!(credits, credits);
        if_some!(weekly_timestamp, weekly_timestamp);
        if_some!(title, title);
        if_some!(time_to_cook, time_to_cook);
        if_some!(servings, servings);
        if_some!(image, image);
        if_some!(ingredients, ingredients);
        if_some!(method, method);
        if_some!(quiz, quiz);

        if let Some(nutrients) = self.nutrients {
            builder = builder.nutrients(
                nutrients
                    .iter()
                    .map(|nutrient| nutrient.into_nutrient())
                    .collect(),
            );
        }

        // Set the remaining Vec fields, then build and return.
        builder.build()
    }
}

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
// #[serde(rename_all = "camelCase")]
// pub struct RequestMethod {
//     /// The steps of the recipe
//     steps: Option<Vec<RequestStep>>,
// }

// impl RequestMethod {
//     /// Tries to convert a RequestMethod into a [`Method`].
//     pub fn into_method(self) -> Result<database::Method, String> {
//         let method = database::Method::new();

//         if let Some(steps) = self.steps {
//             for step in steps {
//                 method = method.add_step(step.into_step()?);
//             }
//         } else {
//             return Err("No steps provided to method.".to_string());
//         }

//         Ok(method)
//     }
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
// #[serde(rename_all = "camelCase")]
// pub struct RequestStep {
//     /// The title of the step.
//     title: Option<String>,
//     /// Each of the substeps in the step
//     substeps: Option<Vec<RequestSubStep>>,
// }

// impl RequestStep {
//     /// Tries to convert a RequestStep into a [`Step`].
//     pub fn into_step(self) -> Result<database::Step, String> {
//         let title = self
//             .title
//             .ok_or_else(|| "No title provided to step.".to_string())?;

//         let mut step = database::Step::new(title);

//         if let Some(substeps) = self.substeps {
//             for substep in substeps {
//                 step = step.add_substep(substep.into_substep()?);
//             }
//         } else {
//             return Err("No substeps provided to step.".to_string());
//         }

//         Ok(step)
//     }
// }

// pub struct RequestSubStep {
//     /// The content of the substep.
//     content: Option<Formattable>,
//     /// An accompanying image to the substep
//     image: Option<Url>,
//     /// Any warning present with this substep
//     warnings: Option<Vec<Warning>>,
//     /// Any information present with this substep
//     infos: Option<Vec<Info>>,
// }

// impl RequestSubStep {
//     /// Tries to convert a RequestSubStep into a [`SubStep`].
//     pub fn into_substep(self) -> Result<database::SubStep, String> {
//         let mut builder = database::SubStep::builder();

//         if let Some(content) = self.content {
//             builder = builder.content(content);
//         }
//         if let Some(image) = self.image {
//             builder = builder.image(image);
//         }
//         if let Some(warnings) = self.warnings {
//             builder = builder.warnings(warnings);
//         }
//         if let Some(infos) = self.infos {
//             builder = builder.infos(infos);
//         }

//         builder.build()
//     }
// }
