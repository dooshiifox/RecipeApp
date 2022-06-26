use crate::v1::types::*;
use actix_api_macros::*;
use actix_web::{get, web, Responder};

#[derive(ActixApiEnum)]
enum BasicRecipeResponse {
    #[success(json)]
    BasicRecipe(BasicRecipe),
    #[failure(
        message = "The specified UUID was not valid. Expected UUIDv4 (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx).",
        json
    )]
    InvalidUuid(String),
}

#[get("/recipe-basic/{uuid}")]
pub async fn uuid(
    client: web::Data<mongodb::Client>,
    path_uuid: web::Path<String>,
) -> impl Responder {
    let path_uuid = path_uuid.into_inner();
    let uuid: Result<Uuid, _> = path_uuid.clone().try_into();
    let uuid = match uuid {
        Ok(uuid) => uuid,
        Err(_) => return BasicRecipeResponse::InvalidUuid(path_uuid),
    };

    let db = client.database("recipes");
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("{}", collection_name);
    }

    BasicRecipeResponse::BasicRecipe(BasicRecipe {
        uuid,
        is_weekly: false,
        title: "".to_string(),
        nutrients: vec![],
        time_to_cook: 0,
        servings: 0,
        image: "".into(),
    })
}
