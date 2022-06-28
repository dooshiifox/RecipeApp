use crate::v1::types::*;
use crate::v1::utils::*;
use actix_api_macros::*;
use actix_web::{get, web, Responder};
use mongodb::bson::doc;
use tracing::{error, trace};

#[derive(ActixApiEnum)]
enum BasicRecipeResponse {
    #[success(json)]
    BasicRecipe(BasicRecipe),
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

#[get("/recipe-basic/{uuid}")]
pub async fn uuid(
    client: web::Data<mongodb::Client>,
    path_uuid: web::Path<String>,
) -> impl Responder {
    // Get the UUID
    let path_uuid = path_uuid.into_inner();
    let uuid: Result<Uuid, _> = path_uuid.clone().try_into();
    trace!("Attempting to get Basic Recipe from UUID: {}", path_uuid);
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(_) => return BasicRecipeResponse::InvalidUuid(path_uuid),
    };

    // Get the recipe from the database
    let db = client.get_collection::<database::Recipe>(Collections::Recipes);
    let recipe = db.find_one(doc! {"_id": uuid}, None).await;
    let recipe = match recipe {
        Ok(Some(recipe)) => recipe,
        Ok(None) => return BasicRecipeResponse::NotFound(uuid),
        Err(err) => {
            let err_id = Uuid::random();
            error!(
                "Error ID: {}\nError getting recipe from database: {}",
                err_id, err
            );
            return BasicRecipeResponse::InternalError(err_id);
        }
    };

    // Convert from db::Recipe to BasicRecipe and return.
    BasicRecipeResponse::BasicRecipe(BasicRecipe::from_recipe(&recipe))
}
