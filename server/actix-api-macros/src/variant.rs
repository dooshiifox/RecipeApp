use crate::{response, attr, statuscode};
use syn::{Ident, Result};
use quote::{format_ident, quote};

/// A Variant holds information on each variant of a Rust enum.
/// 
/// Use as part of [`Enum`].
/// 
/// [`Enum`]: parser::Enum
#[derive(Clone)]
pub struct Variant<'a> {
    /// Syn [`Ident`] of the variant.
    /// 
    /// [`Ident`]: https://docs.rs/syn/1.0.95/syn/struct.Ident.html
    pub variant_ident: &'a Ident,
    /// The number of fields on the variant.
    /// 
    /// ```
    /// enum Foo {
    ///     Bar(i32, i32),  // Some(2)
    ///     Baz(i32),       // Some(1)
    ///     Foob(),         // Some(0)
    ///     Qux             // None
    /// }
    /// ```
    pub field_count: Option<usize>,
    /// The status code to return.
    /// 
    /// If None, the status code should default to `200 OK` if Success or
    /// `400 Bad Request` if Failure.
    pub status: Option<(proc_macro2::Span, u16)>,
    /// The [`Success`] or [`Failure`] attribute macro.
    /// 
    /// [`Success`]: attr::Success
    /// [`Failure`]: attr::Failure
    pub attribute: attr::SuccessFailure
}

impl<'a> Variant<'a> {
    /// Converts the given Variant to a TokenStream which can then be
    /// used in a match statement.
    /// 
    /// `enum_name` should be an [`Ident`] of the name the user gave
    /// to the enum
    /// 
    /// [`Ident`]: https://docs.rs/syn/1.0/syn/struct.Ident.html
    pub fn to_tokenstream(&self, enum_name: &Ident) -> Result<proc_macro2::TokenStream> {
        let name = self.variant_ident;
        let fields = self.field_count;

        // Gets the status_code attribute Ident and the status code.
        let (status_span, status) = match &self.status {
            Some(s) => (Some(&s.0), s.1),
            None => (None, match self.attribute {
                attr::SuccessFailure::Success(_) => 200,
                attr::SuccessFailure::Failure(_) => 400
            })
        };
        
        // Create a bunch of identities for the fields.
        // e.g., variant has 4 fields, we create identifiers `p0`, `p1`, `p2`, `p3`.
        let field_idents = fields.map(|field| (0..field).map(|f| format_ident!("p{}", f)).collect());

        // Gets the JSON the success or the failure attribute should generate
        let json = match &self.attribute {
            attr::SuccessFailure::Success(s) => response::create_success(&field_idents, s),
            attr::SuccessFailure::Failure(f) => response::create_failure(&field_idents, f)
        }?;

        // Creates and returns an Ident for the HttpResponse builder
        let status = statuscode::identifier(status, status_span)?;

        // Constructs the contents of the Match statement
        let contents = quote! {
            ::actix_web::HttpResponse::#status().json(
                ::serde_json::json!({
                    #json
                })
            )
        };
        
        Ok(
            match field_idents {
                None => {
                    // Unit struct, dont have () in match expr
                    quote! {
                        #enum_name::#name => {
                            #contents
                        }
                    }
                },
                Some(fid) => {
                    // Unnamed struct, have (...field_idents) in match expr
                    quote! {
                        #enum_name::#name(#(#fid),*) => {
                            #contents
                        }
                    }
                }
            }
        )
    }
}