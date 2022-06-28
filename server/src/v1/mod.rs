use actix_web::Scope;

mod index;
mod recipe;
mod recipe_basic;
pub mod types;
pub mod utils;

pub fn init(scope: Scope) -> Scope {
    scope
        .service(index::get)
        .service(recipe::post::insert)
        .service(recipe_basic::uuid)
}
