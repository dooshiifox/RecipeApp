use crate::v1::{
    types::{database, *},
    utils,
};
use actix_api_macros::*;
use actix_web::{post, web, HttpRequest, Responder};
use mongodb::bson::doc;

/// The type of recipe sent in the request body.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRecipe {
    /// The unique identifier of the recipe.
    /// If set, overwrites the recipe with the same id.
    uuid: Option<Uuid>,
    /// The date the recipe was added to the database.
    /// If not set, defaults to the time the request was made.
    date_added: Option<Date>,
    /// The date the recipe will/went public. If not yet that date, can only be referred to by id.
    /// If not set, defaults to the time the request was made.
    becomes_public: Option<Date>,
    /// The staff who helped make this recipe.
    authors: Option<Vec<Uuid>>,
    /// A short string crediting the creators of the recipe. Max 400 chars.
    credits: Option<Formattable>,
    /// The date the recipe went weekly. None if never was weekly.
    weekly_timestamp: Option<Date>,
    /// The title of the recipe. Max 80 chars.
    title: Option<String>,
    /// A list of common nutrients found in the recipe. Should be 1-3 long
    nutrients: Option<Vec<Nutrient>>,
    /// The time to cook the recipe, in minutes
    time_to_cook: Option<u16>,
    /// The servings of the recipe.
    servings: Option<u16>,
    /// The URL to the recipe image. Should be on S3
    image: Option<Url>,
    /// The ingredients of the recipe. Max 80chars per ingredient.
    ingredients: Option<Vec<String>>,
    /// The recipe's method
    method: Option<database::Method>,
    /// The quiz information for the end of the recipe
    quiz: Option<database::Quiz>,
}

impl RequestRecipe {
    /// Tries to convert a RequestRecipe into a Recipe.
    pub fn into_recipe(self) -> Result<database::Recipe, String> {
        let mut builder = database::Recipe::builder();

        /// Expands to ```if let Some(value) = self.[field] {
        ///     builder.[setter](value);
        /// }```
        macro_rules! if_some(
            ($field:ident, $setter:ident) => {
                if let Some(value) = self.$field {
                    builder = builder.$setter(value);
                }
            };
        );

        // Set all of the optionable fields on the recipe.
        if_some!(uuid, uuid);
        if_some!(date_added, date_added);
        if_some!(becomes_public, becomes_public);
        if_some!(authors, set_authors);
        if_some!(credits, credits);
        if_some!(weekly_timestamp, weekly_timestamp);
        if_some!(title, title);
        if_some!(nutrients, nutrients);
        if_some!(time_to_cook, time_to_cook);
        if_some!(servings, servings);
        if_some!(image, image);
        if_some!(ingredients, ingredients);
        if_some!(method, method);
        if_some!(quiz, quiz);

        // Set the remaining Vec fields, then build and return.
        builder.build()
    }
}

#[derive(ActixApiEnum)]
enum BasicRecipeResponse {
    /// If the insertion was successful, returns the newly updated [`Recipe`].
    ///
    /// [`Recipe`]: crate::v1::types::database::Recipe
    #[success(message = "Hello World!", json)]
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
    // Important endpoint. Check for authorization before allowing
    // access to insert data.
    if utils::check_user_auth(req).is_err() {
        return BasicRecipeResponse::InvalidAuth;
    }

    // Convert the request recipe to a database recipe.
    let recipe = match body.into_inner().into_recipe() {
        Ok(recipe) => recipe,
        Err(err) => return BasicRecipeResponse::InvalidRequest(err),
    };

    // Get the UUID here so we can use it later to get the entry.
    let recipe_uuid = *recipe.uuid();

    // Insert into the database.
    let result = client
        .database("recipe_db")
        .collection("recipes")
        .insert_one(recipe, None)
        .await;

    if let Err(e) = result {
        let error_uuid = Uuid::random();
        println!("Error UUID: {}\n{:?}", error_uuid, e);
        return BasicRecipeResponse::InternalError(error_uuid);
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
            println!("Error UUID: {}\n{:?}", error_uuid, e);
            return BasicRecipeResponse::InternalError(error_uuid);
        }
        _ => {
            let error_uuid = Uuid::random();
            println!(
                "Error UUID: {}\nCould not find entry in database after inserting: {}",
                error_uuid, recipe_uuid
            );
            return BasicRecipeResponse::InternalError(error_uuid);
        }
    };

    BasicRecipeResponse::Success(recipe)
}
