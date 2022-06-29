use actix_web::Scope;

mod index;
mod recipe;
pub mod types;
pub mod utils;

pub fn init(scope: Scope) -> Scope {
    scope
        .service(index::get)
        .service(recipe::post::insert)
        .service(recipe::get_basic::uuid)
        .service(recipe::get::uuid)
}
