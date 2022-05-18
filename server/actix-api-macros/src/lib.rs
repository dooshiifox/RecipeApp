use syn::{DeriveInput, Result, parse_macro_input};
use proc_macro::TokenStream;
use quote::quote;

mod parser;

#[proc_macro_derive(ActixApiEnum, attributes(success, failure, status_code))]
pub fn macro_api_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_api_enum(&ast);

    gen.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn impl_api_enum(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    // panic!("{:?}", ast);
    let e = parser::parse_variant(ast)?;

    let enum_name = e.name;

    let mut variants = vec![];
    for variant in e.variants {
        variants.push(variant.to_tokenstream(enum_name)?);
    }

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
