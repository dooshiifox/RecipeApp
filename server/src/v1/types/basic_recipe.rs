use crate::v1::types::database::Recipe;
use crate::v1::types::*;

/// A recipe that contains less information than a standard `Recipe` or
/// a database Recipe. This is used to reduce the amount of data that is
/// sent to the client.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicRecipe {
    /// The unique identifier of the recipe.
    pub uuid: Uuid,
    /// The date the recipe was made public.
    pub date_added: Date,
    /// Whether the recipe is *currently* the weekly recipe.
    pub is_weekly: bool,
    /// The title of the recipe. Should be max 80 characters.
    pub title: String,
    /// The nutrients found in the recipe. Should be 1 - 3 long.
    pub nutrients: Vec<SerdeStringNutrient>,
    /// The amount of time, in minutes, to cook the recipe.
    pub time_to_cook: u16,
    /// The number of servings the recipe makes.
    pub servings: u16,
    /// The URL to the image of the recipe.
    pub image: Url,
}

impl BasicRecipe {
    /// Creates a new `BasicRecipe` from a [`database::Recipe`].
    pub fn from_recipe(recipe: &Recipe) -> Self {
        BasicRecipe {
            uuid: recipe.uuid,
            // Return the date it became public instead of the date it
            // was added to the database
            date_added: recipe.becomes_public,
            is_weekly: recipe.is_weekly(),
            title: recipe.title.clone(),
            // Convert Nutrient to SerdeStringNutrient so when sent to the
            // client it will be serialized as a string.
            nutrients: recipe.nutrients.iter().map(|&n| n.into()).collect(),
            time_to_cook: recipe.time_to_cook,
            servings: recipe.servings,
            image: recipe.image.clone(),
        }
    }
}
