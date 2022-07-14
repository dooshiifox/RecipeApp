use crate::v1::types::database::Recipe;
use crate::v1::types::Date;
use crate::v1::utils::collection::*;
use mongodb::options::FindOneOptions;
use mongodb::{bson::doc, Client};
use tracing::{error, trace};

/// Expires after an hour. This needs to be in milliseconds.
const CACHE_EXPIRATION: u64 = 1000 * 60 * 60;

/// Caches the weekly recipe.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WeeklyRecipeGetter {
    /// The date that the weekly recipe was last retrieved from the database.
    last_checked: Date,
    /// The weekly recipe.
    recipe: Option<Recipe>,
}

impl WeeklyRecipeGetter {
    /// Returns the current weekly recipe if the cache is valid
    /// and the recipe is set.
    ///
    /// If this return None, a call to [`WeeklyRecipeGetter::get`] should be made.
    pub fn get_recipe(&self) -> Option<&Recipe> {
        if self.recipe.is_none() || self.is_cache_expired() {
            None
        } else {
            self.recipe.as_ref()
        }
    }

    /// Returns the current weekly recipe.
    ///
    /// If the weekly recipe has not been retrieved from the database or the
    /// cache has expired, will retrieve the weekly recipe from the database.
    /// Else, will return what is cached.
    // Explicit lifetimes here because *sometimes* errors without them for whatever reason..?
    pub async fn get<'a, 'b>(&'a mut self, client: &'b Client) -> Result<&'a Recipe, String> {
        // If the weekly recipe has not been retrieved yet, retrieve it.
        if self.recipe.is_none() || self.is_cache_expired() {
            trace!("Weekly cache expired.");
            self.update(client).await?;
        }

        // Return the weekly recipe.
        self.recipe
            .as_ref()
            .ok_or_else(|| "No weekly recipe found.".to_string())
    }

    /// Checks if the cache is expired.
    pub fn is_cache_expired(&self) -> bool {
        self.last_checked < Date::now() - CACHE_EXPIRATION.into()
    }

    /// Retrieves and updates the weekly recipe, ignoring whether the
    /// cache is valid or not.
    pub async fn update(&mut self, client: &Client) -> Result<(), String> {
        trace!("Updating weekly recipe cache.");
        let db = client.get_collection::<Recipe>(Collections::Recipes);

        // Get the max weekly timestamp
        let find_options = FindOneOptions::builder()
            .sort(doc! {"weeklyTimestamp": -1})
            .build();
        let recipe = db
            .find_one(
                // Not implemented for u64 but *is* implemented for i64,
                // hence the conversion here.
                doc! {"weeklyTimestamp": { "$lt": Date::now().ms() as i64 }},
                find_options,
            )
            .await;
        match recipe {
            Ok(Some(recipe)) => {
                self.recipe = Some(recipe);
                self.last_checked = Date::now();
                Ok(())
            }
            Ok(None) => {
                error!("No weekly recipe found in database");
                Err("No weekly recipe found in database".to_string())
            }
            Err(err) => {
                error!("Error getting weekly recipe from database: {}", err);
                Err("Error getting weekly recipe from database".to_string())
            }
        }
    }
}
