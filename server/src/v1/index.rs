use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

/// An example API call to the server. Returns an expected response.
///
/// This page can be visited with `GET /api/v1/`
#[get("/")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
