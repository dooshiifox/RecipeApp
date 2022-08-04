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
    /// init functions. This is simply an alternative to calling each function
    /// individually with the `scope` variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// mod route_one {
    ///     use actix_web::{web, Scope};
    ///
    ///     mod getter {
    ///         use actix_web::{get, HttpResponse, Responder};
    ///
    ///         #[get("/")]
    ///         pub async fn get() -> impl Responder {
    ///             HttpResponse::Ok().body("Hello World from Route One Getter!")
    ///         }
    ///     }
    ///
    ///     mod poster {
    ///         use actix_web::{post, HttpResponse, Responder};
    ///
    ///         #[post("/")]
    ///         pub async fn post() -> impl Responder {
    ///             HttpResponse::Ok().body("Hello World from Route One Setter!")
    ///         }
    ///     }
    ///
    ///     /// This is a *service generator* for route one,
    ///     /// because it takes a scope and adds its own routes to it.
    ///     pub fn init(scope: Scope) -> Scope {
    ///         scope.service(
    ///             web::scope("/one")
    ///                 .service(getter::get)
    ///                 .service(poster::post),
    ///         )
    ///     }
    /// }
    ///
    /// mod route_two {
    ///     use actix_web::{web, Scope};
    ///
    ///     mod getter {
    ///         use actix_web::{get, HttpResponse, Responder};
    ///
    ///         #[get("/")]
    ///         pub async fn get() -> impl Responder {
    ///             HttpResponse::Ok().body("Hello World from Route Two Getter!")
    ///         }
    ///     }
    ///
    ///     mod poster {
    ///         use actix_web::{post, HttpResponse, Responder};
    ///
    ///         #[post("/")]
    ///         pub async fn post() -> impl Responder {
    ///             HttpResponse::Ok().body("Hello World from Route Two Setter!")
    ///         }
    ///     }
    ///
    ///     /// This is a *service generator* for route two,
    ///     /// because it takes a scope and adds its own routes to it.
    ///     pub fn init(scope: Scope) -> Scope {
    ///         scope.service(
    ///             web::scope("/two")
    ///                 .service(getter::get)
    ///                 .service(poster::post),
    ///         )
    ///     }
    /// }
    ///
    /// use actix_web::{test, web, App, HttpServer};
    /// use crate::v1::Router;
    ///
    /// let app = test::init_service(App::new().service(
    ///         web::scope("/")
    ///             // This is the important part here.
    ///             // It calls the service generators for both routes.
    ///             .service_generator(route_one::init)
    ///             .service_generator(route_two::init)
    ///     )).await;
    ///
    /// // Test `GET /one/`
    /// let req = test::TestRequest::get().uri("/one/").to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert!(resp.status().is_success());
    ///
    /// // Test `POST /one/`
    /// let req = test::TestRequest::post().uri("/one/").to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert!(resp.status().is_success());
    ///
    /// // Test `GET /two/`
    /// let req = test::TestRequest::get().uri("/two/").to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert!(resp.status().is_success());
    ///
    /// // Test `POST /two/`
    /// let req = test::TestRequest::post().uri("/two/").to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert!(resp.status().is_success());
    /// ```
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
