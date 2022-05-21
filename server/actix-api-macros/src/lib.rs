//! Generates the [`actix_web::Responder`] trait for an enum for the purpose
//! of a consistent API.
//! 
//! An enum deriving the `ActixApiEnum` type will have the Responder trait
//! implemented on it with a moderately-opinionted API response.
//! 
//! # Example
//! 
//! ```
//! #[macro_use] extern crate actix_api_macros;
//! use actix_web::{Responder};
//! use serde::Serialize;
//! use serde_json::json;
//! 
//! #[derive(Serialize)]
//! struct Phone {
//!     home: String,
//!     work: String
//! }
//! 
//! #[derive(Serialize)]
//! struct User {
//!     id: u32,
//!     name: String,
//!     phone: Phone,
//! }
//! 
//! #[derive(ActixApiEnum)]
//! enum HelloWorld {
//!     #[success]
//!     Hello,
//!     #[success(message = "Hello, {}")]
//!     HelloName(String),
//!     #[success(message = "Hello User {}!", json)]
//!     HelloUser(String, User),
//!     #[success(json)]
//!     #[status_code(201)]
//!     NewUserAdded(User),
//!     #[failure]
//!     GenericError,
//!     #[failure(message = "Not permitted to view User {}")]
//!     #[status_code(401)]
//!     NotPermitted(u32),
//!     #[failure(message = "User with id {} not found", json)]
//!     UserNotFoundIdJson(u32, u32),
//!     #[failure(message = "Internal Server Error")]
//!     #[status_code(500)]
//!     InternalServerError,
//! }
//! ```
//! 
//! In the above example, each variant can return something different.
//! 
//! The `Hello` variant will respond with simply `{ "success": true }`,
//! meanwhile the `HelloName` variant will respond with
//! `{ "success": true, "message": format!("Hello, {}", p0) }`, where `p0` is
//! the string passed in to the variant. The `HelloUser` variant will respond
//! with `{ "success": true, "message": format!("Hello User {}!", p0), "data": p1 }`,
//! where `p0` is the string passed in to the variant, and `p1` is the User
//! struct passed in to the variant.
//! 
//! Specifying `message` in a `success` or `failure` attribute will cause
//! the `message` field to be added to the JSON response. Its value will be
//! the string passed in to `message` formatted with all arguments of the
//! enum using `format!()`. If `json` is specified, `message` will be formatted
//! with all but the last field.
//! 
//! Specifying `json` in a `success` or `failure` attribute will cause the
//! `data` field to be added to the JSON response. Its value will be the last
//! field of the variant. In the above `HelloUser` example, this is `User`.
//! The type must implement [`serde::Serialize`].
//! 
//! `status_code` can be used to specify the HTTP status code to return.
//! By default, the status code for `success` is `200` and for `failure` is `400`.
//! 
//! For a full example of what the above code would respond with,
//! see the `compiles.rs` example in the `tests` directory.
//! 
//! [`actix_web::Responder`]: actix_web::Responder
//! [`serde::Serialize`]: serde::Serialize

use syn::{DeriveInput, Result, parse_macro_input};
use proc_macro::TokenStream;
use quote::quote;

#[macro_use]
mod attr_macros;

mod parser;
mod attr;
mod response;
mod variant;
mod statuscode;
mod get_if_ident;

/// See top-level crate documentation.
#[proc_macro_derive(ActixApiEnum, attributes(success, failure, status_code))]
pub fn macro_api_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_api_enum(&ast);

    gen.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn impl_api_enum(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // Parse the DeriveInput into an Enum type with most of the info we need
    let e = parser::parse_enum(ast)?;

    // The name the user gave the enum
    let enum_name = e.name;

    // Convert all the enum's variants into a Vec of TokenStreams
    // we can use as a match's statements
    let mut variants = vec![];
    for variant in e.variants {
        variants.push(variant.to_tokenstream(enum_name)?);
    }

    // Implement the Responder trait
    let output = quote! {
        impl ::actix_web::Responder for #enum_name {
            type Body = ::actix_web::body::BoxBody;
        
            fn respond_to(self, _req: &::actix_web::HttpRequest) -> ::actix_web::HttpResponse<Self::Body> {
                match self {
                    #(#variants)*
                }
            }
        }
    };

    Ok(output)
}
