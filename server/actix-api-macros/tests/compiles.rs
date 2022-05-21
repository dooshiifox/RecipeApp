#[macro_use] extern crate actix_api_macros;
use serde::Serialize;
use serde_json::json;
use actix_web::{Responder, body::MessageBody};

#[derive(Serialize)]
struct Phone {
    home: String,
    work: String
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    phone: Phone,
}

#[derive(ActixApiEnum)]
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

/// This is not part of the macro and is simply used to test if it works
/// as expected.
/// 
/// Takes a request, a status code, and a JSON, and compares them.
macro_rules! assert_request {
    ($resp: expr, $status: literal, $json: expr) => {
        let resp = $resp.respond_to(
            &actix_web::test::TestRequest::default().to_http_request()
        );
        assert_eq!(resp.status(), $status);
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json");
        let body = resp.into_body().try_into_bytes().unwrap();
        let bodystr = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(bodystr, $json.to_string());
    }
}

#[test]
fn test_hello() {
    assert_request!(HelloWorld::Hello, 200, json!({
        "success": true
    }));
}

#[test]
fn test_name() {
    assert_request!(HelloWorld::HelloName("Player".to_string()), 200, json!({
        "success": true,
        "message": "Hello, Player"
    }));

    assert_request!(HelloWorld::HelloName("Lil Timmy".to_string()), 200, json!({
        "success": true,
        "message": "Hello, Lil Timmy"
    }));

    assert_request!(HelloWorld::HelloName("".to_string()), 200, json!({
        "success": true,
        "message": "Hello, "
    }));
}

#[test]
fn test_user() {
    assert_request!(HelloWorld::HelloUser("Jimbo".to_string(), User {
        id: 1,
        name: "Jimbo".to_string(),
        phone: Phone {
            home: "555-555-5555".to_string(),
            work: "555-666-7890".to_string()
        }
    }), 200, json!({
        "success": true,
        "message": "Hello User Jimbo!",
        "data": {
            "id": 1,
            "name": "Jimbo",
            "phone": {
                "home": "555-555-5555",
                "work": "555-666-7890"
            }
        }
    }));

    assert_request!(HelloWorld::HelloUser("".to_string(), User {
        id: 0,
        name: "".to_string(),
        phone: Phone {
            home: "".to_string(),
            work: "".to_string()
        }
    }), 200, json!({
        "success": true,
        "message": "Hello User !",
        "data": {
            "id": 0,
            "name": "",
            "phone": {
                "home": "",
                "work": ""
            }
        }
    }));

    // Name does not need to be the same as in the User struct.
    assert_request!(HelloWorld::HelloUser("This has a name".to_string(), User {
        id: 1000000000,
        name: "".to_string(),
        phone: Phone {
            home: "weeeee".to_string(),
            work: "wooooo".to_string()
        }
    }), 200, json!({
        "success": true,
        "message": "Hello User This has a name!",
        "data": {
            "id": 1000000000,
            "name": "",
            "phone": {
                "home": "weeeee",
                "work": "wooooo"
            }
        }
    }));
}

#[test]
fn test_userjson() {
    assert_request!(HelloWorld::HelloUserJson(User {
        id: 1,
        name: "Jimbo".to_string(),
        phone: Phone {
            home: "555-555-5555".to_string(),
            work: "555-666-7890".to_string()
        }
    }), 200, json!({
        "success": true,
        "data": {
            "id": 1,
            "name": "Jimbo",
            "phone": {
                "home": "555-555-5555",
                "work": "555-666-7890"
            }
        }
    }));

    assert_request!(HelloWorld::HelloUserJson(User {
        id: 0,
        name: "".to_string(),
        phone: Phone {
            home: "".to_string(),
            work: "".to_string()
        }
    }), 200, json!({
        "success": true,
        "data": {
            "id": 0,
            "name": "",
            "phone": {
                "home": "",
                "work": ""
            }
        }
    }));

    assert_request!(HelloWorld::HelloUserJson(User {
        id: 1000000000,
        name: "".to_string(),
        phone: Phone {
            home: "weeeee".to_string(),
            work: "wooooo".to_string()
        }
    }), 200, json!({
        "success": true,
        "data": {
            "id": 1000000000,
            "name": "",
            "phone": {
                "home": "weeeee",
                "work": "wooooo"
            }
        }
    }));
}

#[test]
fn test_genericerror() {
    assert_request!(HelloWorld::GenericError, 400, json!({
        "success": false,
    }));
}

#[test]
fn test_not_permitted() {
    assert_request!(HelloWorld::NotPermitted(1), 401, json!({
        "success": false,
        "error": {
            "message": "Not permitted to view User 1"
        }
    }));

    assert_request!(HelloWorld::NotPermitted(0), 401, json!({
        "success": false,
        "error": {
            "message": "Not permitted to view User 0"
        }
    }));

    assert_request!(HelloWorld::NotPermitted(1000000), 401, json!({
        "success": false,
        "error": {
            "message": "Not permitted to view User 1000000"
        }
    }));
}

#[test]
fn test_not_found() {
    assert_request!(HelloWorld::UserNotFoundIdJson(1, 1), 400, json!({
        "success": false,
        "error": {
            "message": "User with id 1 not found",
            "data": 1
        }
    }));

    assert_request!(HelloWorld::UserNotFoundIdJson(0, 0), 400, json!({
        "success": false,
        "error": {
            "message": "User with id 0 not found",
            "data": 0
        }
    }));

    assert_request!(HelloWorld::UserNotFoundIdJson(405, 405), 400, json!({
        "success": false,
        "error": {
            "message": "User with id 405 not found",
            "data": 405
        }
    }));
}

#[test]
fn test_internal_server_error() {
    assert_request!(HelloWorld::InternalServerError, 500, json!({
        "success": false,
        "error": {
            "message": "Internal Server Error"
        }
    }));
}
