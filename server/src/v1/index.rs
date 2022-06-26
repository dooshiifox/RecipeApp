use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
