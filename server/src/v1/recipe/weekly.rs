use crate::id_error;
use crate::v1::types::*;
use crate::v1::utils::*;
use actix_api_macros::*;
use actix_web::{get, web, Responder};
use std::sync::{Arc, Mutex};

#[derive(ActixApiEnum)]
enum WeeklyRecipeResponse {
    #[success(json)]
    WeeklyRecipe(BasicRecipe),
    #[failure(message = "Could not retrieve weekly recipe: {}")]
    WeeklyRecipeNotFound(String),
    #[failure(message = "Internal server error.", json)]
    InternalError(Uuid),
}

// weekly_cacher_lock has an issue with clippy because `await` is called
// on it while the lock is taken. This, as far as I can tell, cannot be avoided.
#[allow(clippy::await_holding_lock)]
#[get("/weekly")]
pub async fn uuid(
    client: web::Data<mongodb::Client>,
    weekly_cacher: web::Data<Arc<Mutex<WeeklyRecipeGetter>>>,
) -> impl Responder {
    let mut weekly_cacher_lock = match weekly_cacher.lock() {
        Ok(weekly_cacher_lock) => weekly_cacher_lock,
        Err(e) => {
            return WeeklyRecipeResponse::InternalError(id_error!(
                "Could not lock weekly recipe cache: {}",
                e
            ));
        }
    };

    let recipe = match weekly_cacher_lock.get(&client).await {
        Ok(recipe) => recipe,
        Err(err) => {
            return WeeklyRecipeResponse::WeeklyRecipeNotFound(err);
        }
    };

    // Convert from db::Recipe to BasicRecipe and return.
    WeeklyRecipeResponse::WeeklyRecipe(BasicRecipe::from_recipe_with_weekly(recipe, true))
}
