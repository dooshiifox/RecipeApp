#[macro_use]
extern crate actix_api_macros;
use serde::Serialize;
#[derive(Serialize)]
struct Phone {
    home: String,
    work: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    phone: Phone,
}

#[derive(ActixApiEnum)]
#[allow(dead_code)]
enum HelloWorld {
    #[success]
    Hello,
    #[success(message = "Hello, {}")]
    HelloName(String),
    #[success(message = "Hello User {}!", json)]
    HelloUser(String, User),
    #[success(json)]
    HelloUserJson(User),
    #[failure]
    GenericError,
    #[failure(message = "Not permitted to view User {}", json = false)]
    #[status_code(401)]
    NotPermitted(u32),
    #[failure(message = "User with id {} not found", json = true)]
    UserNotFoundIdJson(u32, u32),
    #[failure(message = "Internal Server Error")]
    #[status_code(500)]
    InternalServerError,
}
