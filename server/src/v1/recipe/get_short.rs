use crate::v1::types::*;
use crate::v1::utils::*;
use actix_api_macros::*;
use actix_web::{get, web, Responder};
use mongodb::bson::doc;
use tracing::{error, trace};

#[derive(ActixApiEnum)]
enum RecipeResponse {
    #[success(json)]
    Recipe(Recipe),
    #[failure(message = "The specified Short was not found.", json)]
    NotFound(String),
    #[failure(message = "Internal server error.", json)]
    InternalError(Uuid),
}

#[get("/recipe/short/{uuid}")]
pub async fn short(
    client: web::Data<mongodb::Client>,
    weekly_cacher: web::Data<std::sync::Arc<std::sync::Mutex<WeeklyRecipeGetter>>>,
    path_short: web::Path<String>,
) -> impl Responder {
    // Get the short
    let short = path_short.into_inner();
    trace!("Attempting to get Recipe from Short: {}", &short);

    // Get the recipe from the database
    let db = client.get_collection::<database::Recipe>(Collections::Recipes);
    let recipe = db.find_one(doc! {"short": &short}, None).await;
    let recipe = match recipe {
        Ok(Some(recipe)) => recipe,
        Ok(None) => return RecipeResponse::NotFound(short),
        Err(err) => {
            let err_id = Uuid::random();
            error!(
                "Error ID: {}\nError getting recipe from database: {}",
                err_id, err
            );
            return RecipeResponse::InternalError(err_id);
        }
    };

    let mut weekly_cacher_lock = match weekly_cacher.lock() {
        Ok(weekly_cacher_lock) => weekly_cacher_lock,
        Err(e) => {
            return RecipeResponse::InternalError(crate::id_error!(
                "Could not lock weekly recipe cache: {}",
                e
            ));
        }
    };

    // Convert from db::Recipe to BasicRecipe and return.
    RecipeResponse::Recipe(Recipe::from_recipe(&recipe, &mut weekly_cacher_lock, &client).await)
}
