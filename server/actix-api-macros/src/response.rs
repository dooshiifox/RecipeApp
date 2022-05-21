use crate::attr;
use quote::quote;
use syn::Ident;

/// A struct of the common attributes between a `success` and `failure` attribute.
///
/// Used for parsing common attrs in the `success_and_failure_parser` fn.
pub struct CommonAttributes<'a> {
    /// The [`Ident`] (name and span) of the variant.
    ///
    /// [`Ident`]: https://docs.rs/syn/1.0/syn/struct.Ident.html
    #[allow(dead_code)]
    ident: &'a Ident,
    message: &'a Option<attr::Attribute<String>>,
    json: &'a Option<attr::Attribute<bool>>,
}

/// Takes a collection of variant [`Ident`]'s and the common attributes
/// of a `success` and `failure` attribute and returns a tuple of
/// the `message` and `json` fields to use in the response. If the returned
/// `message` or `json` is empty, that field should not be added to the response.
///
/// If `field_idents` is none, there are no fields on the variant (i.e., a
/// unit variant). If `field_idents` is some but the vec is empty, it is an
/// empty unnamed variant.
///
/// [`Ident`]: https://docs.rs/syn/1.0/syn/struct.Ident.html
pub fn success_and_failure_parser(
    field_idents: &Option<Vec<Ident>>,
    sf: CommonAttributes,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let (message, json) = match field_idents {
        None => {
            // This is a unit variant. It means the variant looks something like
            // ```
            // enum Foo {
            //     Bar,
            //     Baz,
            // }
            // ```
            // Since JSON uses the final field as the value, and there is no
            // final field, error if the user attempts to use JSON.
            if let Some(json) = sf.json {
                if json.value {
                    return Err(syn::Error::new(
                        json.ident.span(),
                        "JSON is not supported for unit variants",
                    ));
                }
            }
            match &sf.message {
                Some(attr::Attribute { value: message, .. }) => {
                    // TODO!
                    // When a message is passed such as "Hello {}",
                    // this will error on the `#[derive(ActixApiEnum)]`
                    // instead of the `message = "Hello {}"` section.
                    // Find a way to fix this.
                    // Removing the format!() is not advised as it would create
                    // differences between Unit and Unnamed variants, which
                    // is yucky inconsistency.
                    (
                        quote! {
                            "message": format!(#message),
                        },
                        quote! {},
                    )
                }
                None => (quote! {}, quote! {}),
            }
        }
        Some(field_idents) => {
            // Get the JSON token stream.
            let json = match sf.json {
                Some(attr::Attribute {
                    ident,
                    value: enabled,
                }) if *enabled => {
                    // JSON support uses the final field as the data to send
                    // as JSON. If there is no final field (e.g., `Foo::Bar()`),
                    // then we panic.
                    // The difference between this and the above `None` macth
                    // expression is that Rust supports both `Foo::Bar` and
                    // `Foo::Baz()` (notice the `()`). The second case would cause
                    // this `Some` match expression.
                    if field_idents.is_empty() {
                        return Err(syn::Error::new(
                            ident.span(),
                            "JSON requires at least one field",
                        ));
                    }
                    // Get the final field and set that as the JSON data
                    let last = &field_idents[field_idents.len() - 1];
                    quote! {
                        "data": #last,
                    }
                }
                _ => quote! {},
            };

            match &sf.message {
                Some(attr::Attribute { value: message, .. }) => {
                    // If JSON is enabled, do not inlude that in the args
                    // to `message`
                    // get a slice below so it doesn't complain when we
                    // slice 2 lines later
                    let mut message_field_idents = &field_idents[..];
                    // `is_empty()` is a function on a TokenStream, so instead
                    // of doing an obnoxious json check again we can just see if
                    // the data field exists.
                    if !json.is_empty() {
                        message_field_idents = &field_idents[..field_idents.len() - 1];
                    }

                    (
                        quote! {
                            "message": format!(#message, #(#message_field_idents),*),
                        },
                        json,
                    )
                }
                None => (quote! {}, json),
            }
        }
    };

    Ok((message, json))
}

/// Constructs a JSON response for a `success` attribute.
///
/// See [`success_and_failure_parser`] for more information.
///
/// [`success_and_failure_parser`]: response::success_and_failure_parser
pub fn create_success(
    field_idents: &Option<Vec<Ident>>,
    success: &crate::attr::Success,
) -> syn::Result<proc_macro2::TokenStream> {
    let (message, json) = success_and_failure_parser(
        field_idents,
        CommonAttributes {
            ident: &success.ident,
            message: &success.message,
            json: &success.json,
        },
    )?;

    Ok(quote! {
        "success": true,
        #message
        #json
    })
}

/// Constructs a JSON response for a `failure` attribute.
///
/// See [`success_and_failure_parser`] for more information.
///
/// [`success_and_failure_parser`]: response::success_and_failure_parser
pub fn create_failure(
    field_idents: &Option<Vec<Ident>>,
    failure: &crate::attr::Failure,
) -> syn::Result<proc_macro2::TokenStream> {
    let (message, json) = success_and_failure_parser(
        field_idents,
        CommonAttributes {
            ident: &failure.ident,
            message: &failure.message,
            json: &failure.json,
        },
    )?;

    // In the event neither `message` nor `json` is populated,
    // don't return an empty error field and instead only return
    // the `"success": false` field.
    if !message.is_empty() || !json.is_empty() {
        Ok(quote! {
            "success": false,
            "error": {
                #message
                #json
            }
        })
    } else {
        Ok(quote! {
            "success": false,
        })
    }
}
