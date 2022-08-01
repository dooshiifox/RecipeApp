use actix_web::Scope;

mod index;
mod recipe;
mod search;
pub mod types;
pub mod utils;

/// This trait allows for simpler creation of service generators.
///
/// See [`Router::service_generator`] for more info on it.
pub trait Router
where
    Self: std::marker::Sized,
{
    /// This function allows for simpler creation of a service generator.
    ///
    /// A service generator is a function that takes a scope and
    /// adds services to it.
    ///
    /// Only particularly useful for loading in multiple module's
    /// init functions.
    fn service_generator(self, f: fn(Self) -> Self) -> Self;
}

// Implement onto scope so we can use and chain `scope.service_generator` calls.
impl Router for Scope {
    fn service_generator(self, f: fn(Self) -> Self) -> Self {
        f(self)
    }
}

pub fn init(scope: Scope) -> Scope {
    scope
        .service(index::get)
        .service_generator(recipe::init)
        .service_generator(search::init)
}
