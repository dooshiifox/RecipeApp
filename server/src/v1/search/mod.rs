use actix_web::Scope;

pub mod search;

pub fn init(scope: Scope) -> Scope {
    scope.service(search::search)
}
