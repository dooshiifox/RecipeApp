use crate::v1::types::*;

/// A recipe that contains less information than a standard `Recipe` or
/// a database Recipe. This is used to reduce the amount of data that is
/// sent to the client.
#[derive(serde::Serialize)]
pub struct BasicRecipe {
    /// The unique identifier of the recipe.
    pub uuid: Uuid,
    /// Whether the recipe is *currently* the weekly recipe.
    pub is_weekly: bool,
    /// The title of the recipe. Should be max 80 characters.
    pub title: String,
    /// The nutrients found in the recipe. Should be 1 - 3 long.
    pub nutrients: Vec<Nutrient>,
    /// The amount of time, in minutes, to cook the recipe.
    pub time_to_cook: u16,
    /// The number of servings the recipe makes.
    pub servings: u16,
    /// The URL to the image of the recipe.
    pub image: Url,
}
