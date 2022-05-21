use proc_macro2::Span;
use quote::quote;
use syn::Ident;

/// Converts an Actix-Web StatusCode number into a tokenstream so
/// pre-compile checking can be done on it and to prevent runtime initialising.
pub fn identifier(
    status: u16,
    status_span: Option<&proc_macro2::Span>,
) -> syn::Result<proc_macro2::TokenStream> {
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
        _ => {
            return Err(syn::Error::new(
                *status_span.unwrap(),
                format!("unknown status code: {}", status),
            ))
        }
    };

    let codename = Ident::new(codename, Span::call_site());

    Ok(quote! {#codename})
}
