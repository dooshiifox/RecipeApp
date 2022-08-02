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
pub async fn uuid(weekly_cacher: web::Data<Arc<WeeklyRecipeGetter>>) -> impl Responder {
    let recipe = match weekly_cacher.get().await {
        Ok(recipe) => recipe,
        Err(err) => {
            return WeeklyRecipeResponse::WeeklyRecipeNotFound(err);
        }
    };

    // Convert from db::Recipe to BasicRecipe and return.
    WeeklyRecipeResponse::WeeklyRecipe(BasicRecipe::from_recipe_with_weekly(&recipe, true))
}
