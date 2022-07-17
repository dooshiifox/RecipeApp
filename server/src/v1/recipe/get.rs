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
    #[failure(
        message = "The specified UUID was not valid. Expected UUIDv4 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx).",
        json
    )]
    InvalidUuid(String),
    #[failure(message = "The specified UUID was not found.", json)]
    NotFound(Uuid),
    #[failure(message = "Internal server error.", json)]
    InternalError(Uuid),
}

#[get("/recipe/{uuid}")]
pub async fn uuid(
    client: web::Data<mongodb::Client>,
    weekly_cacher: web::Data<std::sync::Arc<std::sync::Mutex<WeeklyRecipeGetter>>>,
    path_uuid: web::Path<String>,
) -> impl Responder {
    // Get the UUID
    let path_uuid = path_uuid.into_inner();
    let uuid: Result<Uuid, _> = path_uuid.clone().try_into();
    trace!("Attempting to get Recipe from UUID: {}", path_uuid);
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(_) => return RecipeResponse::InvalidUuid(path_uuid),
    };

    // Get the recipe from the database
    let db = client.get_collection::<database::Recipe>(Collections::Recipes);
    let recipe = db.find_one(doc! {"_id": uuid}, None).await;
    let recipe = match recipe {
        Ok(Some(recipe)) => recipe,
        Ok(None) => return RecipeResponse::NotFound(uuid),
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
