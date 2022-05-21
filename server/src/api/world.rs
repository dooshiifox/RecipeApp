use actix_web::{get, Responder, web};
use serde_json::{Value, json};
use actix_api_macros::*;

#[derive(ActixApiEnum)]
enum HelloWorldResponse {
    #[success(message = "Hello {}!")]
    HelloWorld(String),
    #[success(json)]
    HelloWorldJson(Value),
    #[failure(message = "Request was not formatted correctly. `{}`")]
    BadRequest(String),
    #[failure(message = "Unexpected ID: Expected `{}`, got `{}`", json = true)]
    UnexpectedId(String, i32, Value),
    #[failure(message = "Internal Server Error")]
    #[status_code(500)]
    InternalServerError,
}

#[get("/hello/{id}")]
pub async fn get_world(id: web::Path<i32>) -> impl Responder {
    use HelloWorldResponse::*;
    let id = id.into_inner();

    match id {
        1 => HelloWorld("Hello".to_string()),
        2 => HelloWorld("World".to_string()),
        3 => BadRequest("That was a 3. Wrong answer.".to_string()),
        4..=8 => HelloWorldJson(json!({
            "provided_id": id,
            "sub3": id - 3,
            "hello": "world!"
        })),
        9.. => UnexpectedId("1..=8".to_string(), id, json!({
            "provided_id": id,
            "expected_id": {
                "min": 1,
                "max": 8
            }
        })),
        // Matches <= 0
        _ => InternalServerError
    }
}
