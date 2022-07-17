use crate::v1::types::database::Recipe;
use crate::v1::types::*;
use crate::WeeklyRecipeGetter;
use mongodb::Client;

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
    /// The short name of the recipe, used in the URL.
    ///
    /// e.g., `/recipe/chicken-tikka-masala`, `chicken-tikka-masala` is this string.
    pub short: String,
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
    /// The gradient of the recipe.
    pub gradient: Gradient,
}

impl BasicRecipe {
    /// Creates a new `BasicRecipe` from a [`database::Recipe`].
    pub async fn from_recipe(
        recipe: &Recipe,
        weekly_getter: &mut WeeklyRecipeGetter,
        db_client: &Client,
    ) -> Self {
        BasicRecipe {
            uuid: recipe.uuid,
            // Return the date it became public instead of the date it
            // was added to the database
            date_added: recipe.becomes_public,
            is_weekly: recipe.is_weekly(weekly_getter, db_client).await,
            short: recipe.short.clone(),
            title: recipe.title.clone(),
            // Convert Nutrient to SerdeStringNutrient so when sent to the
            // client it will be serialized as a string.
            nutrients: recipe.nutrients.iter().map(|&n| n.into()).collect(),
            time_to_cook: recipe.time_to_cook,
            servings: recipe.servings,
            image: recipe.image.clone(),
            gradient: recipe.gradient.clone(),
        }
    }

    /// Creates a new `BasicRecipe` from a [`database::Recipe`], setting
    /// the `is_weekly` field manually.
    pub fn from_recipe_with_weekly(recipe: &Recipe, is_weekly: bool) -> Self {
        BasicRecipe {
            uuid: recipe.uuid,
            // Return the date it became public instead of the date it
            // was added to the database
            date_added: recipe.becomes_public,
            is_weekly,
            short: recipe.short.clone(),
            title: recipe.title.clone(),
            // Convert Nutrient to SerdeStringNutrient so when sent to the
            // client it will be serialized as a string.
            nutrients: recipe.nutrients.iter().map(|&n| n.into()).collect(),
            time_to_cook: recipe.time_to_cook,
            servings: recipe.servings,
            image: recipe.image.clone(),
            gradient: recipe.gradient.clone(),
        }
    }
}
