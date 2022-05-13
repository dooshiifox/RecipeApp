use serde_json::Value;
use actix_web::body::BoxBody;
use actix_web::{HttpResponse, HttpRequest, Responder};
use std::fmt::Debug;
use serde::Serialize;
use serde_json::json;

/// A struct representing an API response.
/// 
/// The type `S` is the type of the response body if the response is successful.
/// 
/// The type `M` is the message the error returns if it is not successful.
/// The type `E` is the type of the response body if the response is not successful.
/// 
/// The purpose of this struct is to create a unified API design for the app.
/// 
/// A successful result returns the following JSON when `to_response` is called.
/// 
/// ```
/// {
///     success: true,
///     data: serde::to_string(S),
/// }
/// ```
/// 
/// An unsuccessful result returns the following JSON when `to_response` is called.
/// 
/// ```
/// {
///     success: false,
///     error: {
///         message: serde::to_string(M),
///         error: serde::to_string(E),
///     }
/// }
/// ```
pub enum ApiResult<S, M, E>
where
    S: Debug + Serialize,
    M: Debug + Serialize,
    E: Debug + Serialize
{
    Success(S),
    Error(M, Option<E>)
}


impl<S, M, E> ApiResult<S, M, E>
where
    S: Debug + Serialize,
    M: Debug + Serialize,
    E: Debug + Serialize
{
    /// Creates a new ApiResult with the given success and body.
    pub fn new(res: Result<S, (M, E)>) -> Self {
        match res {
            Ok(s) => ApiResult::Success(s),
            Err(e) => ApiResult::Error(e.0, Some(e.1))
        }
    }

    /// Creates a new ApiResult from a `Result`.
    /// 
    /// In the event of an `Err`, there will only be a message present.
    /// If you wish to have some data go with it, try `ApiResult::new`.
    pub fn new_msg(res: Result<S, M>) -> Self {
        match res {
            Ok(s) => ApiResult::Success(s),
            Err(m) => ApiResult::Error(m, None)
        }
    }

    /// Creates a new successful ApiResult with the given body.
    pub fn success(body: S) -> Self {
        ApiResult::Success(body)
    }

    /// Creates a new unsuccessful ApiResult with the given error message and body.
    pub fn error(message: M, body: E) -> Self {
        ApiResult::Error(message, Some(body))
    }

    /// Creates a new unsuccessful ApiResult with the given error message.
    pub fn error_msg(message: M) -> Self {
        ApiResult::Error(message, None)
    }

    /// Create this when you know a type can not be an error.
    /// Otherwise, use `ApiResult::success` as the type it returns
    /// works well with errors.
    pub fn known_success(body: S) -> ApiResult<S, (), ()> {
        ApiResult::Success(body)
    }

    /// Create this when you know a type can not be a success.
    /// Otherwise, use `ApiResult::error` as the type it returns
    /// works well with success and a lack of body.
    pub fn known_error(message: M, body: E) -> ApiResult<(), M, E> {
        ApiResult::Error(message, Some(body))
    }

    /// Create this when you know a type can not be a success and can not have a body.
    /// Otherwise, use `ApiResult::error_msg` as the type it returns
    /// works well with success and a body.
    pub fn known_error_msg(message: M) -> ApiResult<(), M, ()> {
        ApiResult::Error(message, None)
    }

    /// Returns it as a json.
    pub fn as_json(&self) -> Value {
        match self {
            ApiResult::Success(data) => json!({
                "success": true,
                "data": data
            }),
            ApiResult::Error(msg, data) => {
                if let Some(data) = data {
                    json!({
                        "success": false,
                        "error": {
                            "message": msg,
                            "data": data
                        }
                    })
                } else {
                    json!({
                        "success": false,
                        "error": {
                            "message": msg
                        }
                    })
                }
            },
        }
    }
}


// Implement the Responder trait on API Result so it can be used as a response.
impl<S, M, E> Responder for ApiResult<S, M, E>
where
    S: Debug + Serialize,
    M: Debug + Serialize,
    E: Debug + Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let json = self.as_json();

        // Return the result as a json
        HttpResponse::Ok().json(json)
    }
}
