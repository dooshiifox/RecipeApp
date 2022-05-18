use proc_macro2::Span;
use syn::Ident;
use syn::DeriveInput;
use syn::Result;
use quote::{quote, format_ident};

#[derive(Clone)]
pub struct Success {
    pub message: Option<String>,
    pub json: bool,
}

#[derive(Clone)]
pub struct Failure {
    pub message: Option<String>,
    pub json: bool,
}

#[derive(Clone)]
pub enum SuccessFailure {
    Success(Success),
    Failure(Failure)
}

pub struct Enum<'a> {
    pub name: &'a Ident,
    pub variants: Vec<Variant<'a>>,
}

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
    pub status: Option<(Ident, u16)>,
    /// The [`Success`] or [`Failure`] attribute macro.
    /// 
    /// [`Success`]: parser::Success
    /// [`Failure`]: parser::Failure
    pub attribute: SuccessFailure
}

pub fn parse_variant(ast: &DeriveInput) -> Result<Enum> {
    let mut e = Enum {
        name: &ast.ident,
        variants: vec![]
    };

    if let syn::Data::Enum(enum_int) = &ast.data {
        for variant in &enum_int.variants {
            // enum ABC {
            //    /// Unnamed Field
            //    A(i32),
            //    /// Unit Field
            //    B,
            //    /// Named Field
            //    C {
            //        x: i32,
            //        y: i32
            //    }
            // }
            let field_count = match &variant.fields {
                syn::Fields::Unnamed(fields) => Some(fields.unnamed.len()),
                syn::Fields::Unit => None,
                _ => panic!("Only unnamed fields are supported")
            };

            let (status, attribute) = parse_variant_attribute(&variant.attrs)?;

            e.variants.push(Variant {
                variant_ident: &variant.ident,
                field_count,
                attribute,
                status,
            });
        }
    }

    Ok(e)
}

macro_rules! meta_attr {
    ($name: literal as str in $metanamevalue: ident) => {{
        match &$metanamevalue {
            syn::Meta::NameValue(name_value) => {
                if name_value.path.is_ident($name) {
                    match &name_value.lit {
                        syn::Lit::Str(lit) => Some(lit.value()),
                        _ => panic!("Expected string literal for {}", $name)
                    }
                } else {
                    None
                }
            },
            _ => None
        }
    }};
    ($name: literal exists in $metanamevalue: ident) => {{
        $metanamevalue.path.is_ident($name)
    }};
    (get ident $name:literal from $metanamevalue:ident) => {{
        &$metanamevalue.path.segments.last().unwrap().ident
    }};
    ($name: literal exists or as bool in $metanamevalue: ident) => {{
        match &$metanamevalue {
            syn::Meta::NameValue(name_value) => {
                if name_value.path.is_ident($name) {
                    if let syn::Lit::Bool(b) = &name_value.lit {
                        Some(b.value)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            syn::Meta::Path(path) => {
                if path.is_ident($name) {
                    Some(true)
                } else {
                    None
                }
            }
            _ => None
        }
    }};
    (literal int as $nestedmeta: expr) => {{
        let nm = $nestedmeta;
        match &nm {
            syn::NestedMeta::Lit(syn::Lit::Int(i)) => Some(i.base10_parse::<u16>().unwrap()),
            _ => None
        }
    }};
}

fn parse_variant_attribute(attrs: &[syn::Attribute]) -> Result<(Option<(Ident, u16)>, SuccessFailure)> {
    let mut success_failure = None;
    let mut status = None;

    for attr in attrs {
        if let Ok(syn::Meta::List(name_value)) = attr.parse_meta() {
            if meta_attr!("success" exists in name_value) {
                if success_failure.is_some() {
                    panic!("Only one success or failure attribute is allowed");
                }
                
                let mut s = Success {
                    message: None,
                    json: false,
                };

                for meta in name_value.nested {
                    if let syn::NestedMeta::Meta(meta) = meta {
                        if let Some(j) = meta_attr!("json" exists or as bool in meta) {
                            s.json = j;
                        } else if let Some(msg) = meta_attr!("message" as str in meta) {
                            s.message = Some(msg);
                        } else {
                            panic!("Unknown attribute @1: {:?}", meta);
                        }
                    } else {
                        panic!("Unknown attribute @2: {:?}", meta);
                    }
                }

                success_failure = Some(SuccessFailure::Success(s));
            } else if meta_attr!("failure" exists in name_value) {
                if success_failure.is_some() {
                    panic!("Only one success or failure attribute is allowed");
                }
                
                let mut f = Failure {
                    message: None,
                    json: false,
                };

                for meta in name_value.nested {
                    if let syn::NestedMeta::Meta(meta) = meta {
                        if let Some(j) = meta_attr!("json" exists or as bool in meta) {
                            f.json = j;
                        } else if let Some(msg) = meta_attr!("message" as str in meta) {
                            f.message = Some(msg);
                        } else {
                            panic!("Unknown attribute @1: {:?}", meta);
                        }
                    } else {
                        panic!("Unknown attribute @2: {:?}", meta);
                    }
                }

                success_failure = Some(SuccessFailure::Failure(f));
            } else if meta_attr!("status_code" exists in name_value) {
                let n = meta_attr!(literal int as name_value.nested.first().unwrap());
                status = Some((
                    meta_attr!(get ident "status_code" from name_value).clone(),
                    n.unwrap()
                ));
            }
        }
    }

    Ok((status, success_failure.expect("No success or failure attribute found")))
}

impl<'a> Variant<'a> {
    pub fn to_tokenstream(&self, enum_name: &Ident) -> Result<proc_macro2::TokenStream> {
        let name = self.variant_ident;
        let fields = self.field_count;

        let (status_ident, status) = match &self.status {
            Some(s) => (Some(&s.0), s.1),
            None => (None, match self.attribute {
                SuccessFailure::Success(_) => 200,
                SuccessFailure::Failure(_) => 400
            })
        };
        
        // Create a bunch of identities for the fields.
        // e.g., variant has 4 fields, we create identifiers `p0`, `p1`, `p2`, `p3`.
        let field_idents = fields.map(|field| (0..field).map(|f| format_ident!("p{}", f)).collect());

        let json = match &self.attribute {
            SuccessFailure::Success(s) => create_success(&field_idents, s),
            SuccessFailure::Failure(f) => create_failure(&field_idents, f)
        }?;

        let status = statuscode_tokenstream(status, status_ident)?;
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
                    quote! {
                        #enum_name::#name => {
                            #contents
                        }
                    }
                },
                Some(fid) => {
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

fn create_success(
    field_idents: &Option<Vec<Ident>>,
    s: &Success
) -> Result<proc_macro2::TokenStream> {
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
            if s.json {
                panic!("JSON is not supported for unit variants");
            }
            
            match &s.message {
                Some(message) => {
                    (quote! {
                        "message": #message,
                    }, quote! {})
                },
                None => {
                    (quote! {}, quote! {})
                }
            }
        },
        Some(field_idents) => {
            // Get the JSON token stream.
            let json = if s.json {
                // JSON support uses the final field as the data to send
                // as JSON. If there is no final field (e.g., `Foo::Bar()`),
                // then we panic.
                if field_idents.is_empty() {
                    panic!("JSON requires at least 1 field.")
                }

                // Get the final field and set that as the JSON data
                let last = &field_idents[field_idents.len() - 1];
                quote! {
                    "data": #last,
                }
            } else {
                quote! {}
            };

            match &s.message {
                Some(message) => {
                    // If JSON is enabled, do not inlude that in the args
                    // to `message`
                    // get a slice below so it doesn't complain when we
                    // slice 2 lines later
                    let mut message_field_idents = &field_idents[..];
                    if s.json {
                        message_field_idents = &field_idents[..field_idents.len() - 1];
                    }

                    (quote! {
                        "message": format!(#message, #(#message_field_idents),*),
                    }, json)
                },
                None => {
                    (quote! {}, json)
                }
            }
        }
    };

    Ok(quote! {
        "success": true,
        #message
        #json
    })
}

fn create_failure(
    field_idents: &Option<Vec<Ident>>,
    f: &Failure
) -> Result<proc_macro2::TokenStream> {
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
            if f.json {
                panic!("JSON is not supported for unit variants");
            }
            
            match &f.message {
                Some(message) => {
                    (quote! {
                        "message": #message,
                    }, quote! {})
                },
                None => {
                    (quote! {}, quote! {})
                }
            }
        },
        Some(field_idents) => {
            // Get the JSON token stream.
            let json = if f.json {
                // JSON support uses the final field as the data to send
                // as JSON. If there is no final field (e.g., `Foo::Bar()`),
                // then we panic.
                if field_idents.is_empty() {
                    panic!("JSON requires at least 1 field.")
                }

                // Get the final field and set that as the JSON data
                let last = &field_idents[field_idents.len() - 1];
                quote! {
                    "data": #last,
                }
            } else {
                quote! {}
            };

            match &f.message {
                Some(message) => {
                    // If JSON is enabled, do not inlude that in the args
                    // to `message`
                    // get a slice below so it doesn't complain when we
                    // slice 2 lines later
                    let mut message_field_idents = &field_idents[..];
                    if f.json {
                        message_field_idents = &field_idents[..field_idents.len() - 1];
                    }

                    (quote! {
                        "message": format!(#message, #(#message_field_idents),*),
                    }, json)
                },
                None => {
                    (quote! {}, json)
                }
            }
        }
    };

    Ok(quote! {
        "success": false,
        "error": {
            #message
            #json
        }
    })
}

/// Converts an Actix-Web StatusCode number into a tokenstream so
/// pre-compile checking can be done on it and to prevent runtime initialising.
fn statuscode_tokenstream(status: u16, status_ident: Option<&Ident>) -> syn::Result<proc_macro2::TokenStream> {
    let codename = match status {
        100 => "Continue",
        101 => "SwitchingProtocols",
        102 => "Processing",

        200 => "Ok",
        201 => "Created",
        202 => "Accepted",
        203 => "NonAuthoritativeInformation",

        204 => "NoContent",
        205 => "ResetContent",
        206 => "PartialContent",
        207 => "MultiStatus",
        208 => "AlreadyReported",

        300 => "MultipleChoices",
        301 => "MovedPermanently",
        302 => "Found",
        303 => "SeeOther",
        304 => "NotModified",
        305 => "UseProxy",
        307 => "TemporaryRedirect",
        308 => "PermanentRedirect",

        400 => "BadRequest",
        404 => "NotFound",
        401 => "Unauthorized",
        402 => "PaymentRequired",
        403 => "Forbidden",
        405 => "MethodNotAllowed",
        406 => "NotAcceptable",
        407 => "ProxyAuthenticationRequired",
        408 => "RequestTimeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "LengthRequired",
        412 => "PreconditionFailed",
        428 => "PreconditionRequired",
        413 => "PayloadTooLarge",
        414 => "UriTooLong",
        415 => "UnsupportedMediaType",
        416 => "RangeNotSatisfiable",
        417 => "ExpectationFailed",
        422 => "UnprocessableEntity",
        429 => "TooManyRequests",
        431 => "RequestHeaderFieldsTooLarge",
        451 => "UnavailableForLegalReasons",

        // 418 => "ImATeapot",
        // 421 => "MisdirectedRequest",
        // 423 => "Locked",
        // 424 => "FailedDependency",
        // 426 => "UpgradeRequired",

        500 => "InternalServerError",
        501 => "NotImplemented",
        502 => "BadGateway",
        503 => "ServiceUnavailable",
        504 => "GatewayTimeout",
        505 => "VersionNotSupported",
        506 => "VariantAlsoNegotiates",
        507 => "InsufficientStorage",
        508 => "LoopDetected",
        // 510 => "NotExtended",
        // 511 => "NetworkAuthenticationRequired",
        _ => return Err(syn::Error::new(
            status_ident.unwrap().span(),
            format!("unknown status code: {}", status)
        )),
    };

    let codename = Ident::new(codename, Span::call_site());

    Ok(quote! {#codename})
}
