use crate::v1::types::database::Recipe;
use crate::v1::types::Date;
use crate::v1::utils::collection::*;
use mongodb::options::FindOneOptions;
use mongodb::{bson::doc, Client};
use std::sync::RwLock;
use tracing::{error, trace};

/// Expires after an hour. This needs to be in milliseconds.
const CACHE_EXPIRATION: u64 = 1000 * 60 * 60;

/// Caches the weekly recipe.
#[derive(Debug)]
pub struct WeeklyRecipeGetter {
    /// The date that the weekly recipe was last retrieved from the database.
    last_checked: RwLock<Date>,
    /// The weekly recipe.
    recipe: RwLock<Option<Recipe>>,
    /// A reference to the MongoDB client.
    client: Client,
}

impl WeeklyRecipeGetter {
    /// Creates a new WeeklyRecipeGetter
    pub fn new(client: Client) -> Self {
        Self {
            last_checked: RwLock::new(Date::default()),
            recipe: RwLock::new(None),
            client,
        }
    }

    /// Returns the current weekly recipe if the cache is valid
    /// and the recipe is set.
    ///
    /// If this return None, a call to [`WeeklyRecipeGetter::get`] should be made.
    pub fn get_recipe(&self) -> Option<Recipe> {
        if self.is_cache_expired() {
            return None;
        }

        self.recipe.read().ok()?.clone()
    }

    /// Returns the current weekly recipe.
    ///
    /// If the weekly recipe has not been retrieved from the database or the
    /// cache has expired, will retrieve the weekly recipe from the database.
    /// Else, will return what is cached.
    // Explicit lifetimes here because *sometimes* errors without them for whatever reason..?
    pub async fn get(&self) -> Result<Recipe, String> {
        if let Some(recipe) = self.get_recipe() {
            return Ok(recipe);
        }

        // If the weekly recipe has not been retrieved yet, retrieve it.
        trace!("Weekly cache expired.");
        self.update().await?;
        if let Some(recipe) = self.get_recipe() {
            return Ok(recipe);
        }

        Err("Weekly recipe was not found.".to_string())
    }

    /// Checks if the cache is expired.
    pub fn is_cache_expired(&self) -> bool {
        let last_checked = match self.last_checked.read() {
            Ok(last_checked) => *last_checked,
            Err(_) => return true,
        };
        last_checked < Date::now() - CACHE_EXPIRATION.into()
    }

    /// Retrieves and updates the weekly recipe, ignoring whether the
    /// cache is valid or not.
    pub async fn update(&self) -> Result<(), String> {
        trace!("Updating weekly recipe cache.");

        // Get the max weekly timestamp
        let find_options = FindOneOptions::builder()
            .sort(doc! {"weeklyTimestamp": -1})
            .build();

        let recipe = {
            // Get the client
            let db = self.client.get_collection::<Recipe>(Collections::Recipes);
            db.find_one(
                // Not implemented for u64 but *is* implemented for i64,
                // hence the conversion here.
                doc! {"weeklyTimestamp": { "$lt": Date::now().ms() as i64 }},
                find_options,
            )
            .await
        };

        match recipe {
            Ok(Some(recipe)) => {
                // Write to the cache
                let mut recipe_write = self
                    .recipe
                    .write()
                    .map_err(|_| "Could not write recipe to cache.".to_string())?;
                *recipe_write = Some(recipe);

                let mut checked_write = self
                    .last_checked
                    .write()
                    .map_err(|_| "Could not write recipe to cache.".to_string())?;
                *checked_write = Date::now();
                // Empty for success
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
