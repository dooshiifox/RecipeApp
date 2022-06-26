use actix_web::Scope;

mod index;
mod recipe;
mod recipe_basic;
mod types;
mod utils;

pub fn init(scope: Scope) -> Scope {
    scope
        .service(index::get)
        .service(recipe::insert)
        .service(recipe_basic::uuid)
}
