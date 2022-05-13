use actix_web::{get, Responder, web};
use crate::api::apires::*;
use serde_json::json;

#[get("/hello/{id}")]
pub async fn get_world(id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    match id {
        1 => ApiResult::success("Hello"),
        2 => ApiResult::success("World"),
        3 => ApiResult::error_msg("That was a 3. Wrong answer."),
        4..=8 => ApiResult::error("Seriously?", json!({
            "provided_id": id,
            "sub3": id - 3,
            "nestTest": {
                "ok": "it works",
                "butDoArrays": [
                    "work",
                    "too",
                    "?",
                    {
                        "doThey?": true,
                        "woohoo!": null
                    }
                ]
            }
        })),
        _ => ApiResult::error_msg("Sorry but that number is higher than I can count :(")
    }
}
