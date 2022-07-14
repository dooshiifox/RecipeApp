use crate::v1::recipe::RequestRecipe;
use crate::v1::{types::*, utils::*};
use actix_api_macros::*;
use actix_web::{post, web, HttpRequest, Responder};
use mongodb::bson::doc;
use tracing::{error, trace};

#[derive(ActixApiEnum)]
#[allow(clippy::large_enum_variant)]
enum RecipeResponse {
    /// If the insertion was successful, returns the newly updated [`Recipe`].
    ///
    /// [`Recipe`]: crate::v1::types::database::Recipe
    #[success(message = "Successfully inserted into the database", json)]
    Success(database::Recipe),
    /// Returns if the user provided an invalid authorization token.
    #[failure(
        message = "Invalid authorization. Either there is no Authorization header or the bearer token is invalid."
    )]
    #[status_code(401)]
    InvalidAuth,
    /// Returns if the user provided an invalid request body.
    #[failure(message = "Invalid request body: `{}`.")]
    #[status_code(400)]
    InvalidRequest(String),
    /// In the event an issue in the server occured, returns this error.
    /// Contains a UUID that can be used to identify the issue.
    #[failure(message = "Internal server error. Error UUID: `{}`")]
    #[status_code(500)]
    InternalError(Uuid),
}

/// A request to insert or update a recipe.
#[post("/recipe")]
pub async fn insert(
    req: HttpRequest,
    client: web::Data<mongodb::Client>,
    body: web::Json<RequestRecipe>,
) -> impl Responder {
    trace!("Attempting to insert recipe.");
    // Important endpoint. Check for authorization before allowing
    // access to insert data.
    if check_user_auth(req).is_err() {
        trace!("Invalid authorization attempt.");
        return RecipeResponse::InvalidAuth;
    }

    // Convert the request recipe to a database recipe.
    let recipe = match body.into_inner().into_recipe() {
        Ok(recipe) => recipe,
        Err(err) => {
            trace!(
                "Could not insert recipe due to invalid request body: {}",
                err
            );
            return RecipeResponse::InvalidRequest(err);
        }
    };

    // Get the UUID here so we can use it later to get the entry.
    let recipe_uuid = *recipe.uuid();

    // Insert into the database.
    let result = client
        .get_collection(Collections::Recipes)
        .insert_one(recipe, None)
        .await;

    if let Err(e) = result {
        let error_uuid = Uuid::random();
        error!("Error UUID: {}\n{:?}", error_uuid, e);
        return RecipeResponse::InternalError(error_uuid);
    }

    // Get the recipe from the database.
    let result = client
        .database("recipe_db")
        .collection("recipes")
        .find_one(doc! {"_id": recipe_uuid}, None)
        .await;

    let recipe = match result {
        Ok(Some(recipe)) => recipe,
        Err(e) => {
            let error_uuid = Uuid::random();
            error!("Error UUID: {}\n{:?}", error_uuid, e);
            return RecipeResponse::InternalError(error_uuid);
        }
        _ => {
            let error_uuid = Uuid::random();
            error!(
                "Error UUID: {}\nCould not find entry in database after inserting: {}",
                error_uuid, recipe_uuid
            );
            return RecipeResponse::InternalError(error_uuid);
        }
    };

    trace!("Successfully inserted/updated recipe {}.", recipe_uuid);
    RecipeResponse::Success(recipe)
}
