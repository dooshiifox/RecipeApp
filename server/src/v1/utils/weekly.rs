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
    /// Returns the current weekly recipe.
    ///
    /// If the weekly recipe has not been retrieved from the database or the
    /// cache has expired, will retrieve the weekly recipe from the database.
    /// Else, will return what is cached.
    pub async fn get(&mut self, client: Client) -> Option<&Recipe> {
        // If the weekly recipe has not been retrieved yet, retrieve it.
        if self.recipe.is_none() || self.is_cache_expired() {
            trace!("Weekly cache expired.");
            self.update(client).await;
        }

        // Return the weekly recipe.
        self.recipe.as_ref()
    }

    /// Checks if the cache is expired.
    pub fn is_cache_expired(&self) -> bool {
        self.last_checked < Date::now() - CACHE_EXPIRATION.into()
    }

    /// Retrieves and updates the weekly recipe, ignoring whether the
    /// cache is valid or not.
    pub async fn update(&mut self, client: Client) {
        trace!("Updating weekly recipe cache.");
        let db = client.get_collection::<Recipe>(Collections::Recipes);

        // Get the max weekly timestamp
        let find_options = FindOneOptions::builder()
            .sort(doc! {"weekly_timestamp": -1})
            .build();
        let recipe = db
            .find_one(
                doc! {"weekly_timestamp": { "$lt": Date::now() }},
                find_options,
            )
            .await;
        match recipe {
            Ok(Some(recipe)) => {
                self.recipe = Some(recipe);
                self.last_checked = Date::now();
            }
            Ok(None) => {
                error!("No weekly recipe found in database");
            }
            Err(err) => {
                error!("Error getting weekly recipe from database: {}", err);
            }
        }
    }
}
