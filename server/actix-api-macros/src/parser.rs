use crate::{attr, variant};
use syn::{Ident, DeriveInput, Result};

pub struct Enum<'a> {
    pub name: &'a Ident,
    pub variants: Vec<variant::Variant<'a>>,
}

/// Parses a DeriveInput Enum and returns an Enum struct.
/// 
/// Errors if the input is not an Enum.
pub fn parse_enum(ast: &DeriveInput) -> Result<Enum> {
    let mut e = Enum {
        name: &ast.ident,
        variants: vec![]
    };

    if let syn::Data::Enum(enum_int) = &ast.data {
        for variant in &enum_int.variants {
            // enum ABC {
            //    /// Unnamed Field
            //    A(i32),
            //    /// Still an unnamed Field
            //    A2(),
            //    /// Unit Field
            //    B,
            //    /// Named Field
            //    C {
            //        x: i32,
            //        y: i32
            //    }
            // }
            // Unit variants are a None for their field count.
            // Unnamed variants are a Some with the number of fields.
            // Named variants are an error
            let field_count = match &variant.fields {
                syn::Fields::Unnamed(fields) => Some(fields.unnamed.len()),
                syn::Fields::Unit => None,
                syn::Fields::Named(_) => return Err(syn::Error::new(
                    variant.ident.span(),
                    "Only unnamed fields are supported"
                ))
            };

            // Get the status code and attributes for the variant
            let (status, attribute) = attr::parse_variant_attribute(
                &variant.attrs,
                variant.ident.span()
            )?;

            // Add the variant with its field count, status code, and attributes
            // to the enum data
            e.variants.push(variant::Variant {
                variant_ident: &variant.ident,
                field_count,
                attribute,
                status,
            });
        }
    } else {
        // Not an enum
        return Err(syn::Error::new(
            ast.ident.span(),
            "Only Enums are supported"
        ));
    }

    Ok(e)
}
