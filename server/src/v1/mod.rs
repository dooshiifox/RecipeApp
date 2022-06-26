use actix_web::Scope;

mod index;
mod recipe_basic;
mod types;

use index::get;
use recipe_basic::uuid;

pub fn init(scope: Scope) -> Scope {
    scope.service(get).service(uuid)
}
