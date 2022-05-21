use crate::attr_macros::AttrFound;

use crate::get_if_ident::GetIfIdent;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Ident, Meta, NestedMeta};

/// A typing representing an attribute field on a variant.
#[derive(Clone)]
pub struct Attribute<T> {
    pub ident: Ident,
    pub value: T,
}

impl<T> Attribute<T> {
    pub fn new(ident: Ident, value: T) -> Self {
        Self { ident, value }
    }
}

#[derive(Clone)]
pub struct Success {
    pub ident: Ident,
    pub message: Option<Attribute<String>>,
    pub json: Option<Attribute<bool>>,
}

#[derive(Clone)]
pub struct Failure {
    pub ident: Ident,
    pub message: Option<Attribute<String>>,
    pub json: Option<Attribute<bool>>,
}

#[derive(Clone)]
pub enum SuccessFailure {
    Success(Success),
    Failure(Failure),
}

pub fn parse_variant_attribute(
    attrs: &[syn::Attribute],
    unfound_span: proc_macro2::Span,
) -> syn::Result<(Option<(proc_macro2::Span, u16)>, SuccessFailure)> {
    let mut success_failure = None;
    let mut status = None;

    for attr in attrs {
        let attr = attr.parse_meta().unwrap();

        // Check if the attribute is #[success]
        if let Some((ident, sub_attrs)) = attr!(attr == #[success(..)?]).found() {
            // Check if a success or failure attribute already exists
            if success_failure.is_some() {
                return err!(ident, "Only one success or failure attribute is allowed");
            }

            // Create a default Success struct.
            let mut success = Success {
                ident: ident.clone(),
                message: None,
                json: None,
            };

            // Check that the Success attribute has sub-attributes
            // If it doesnt (i.e., just #[success]) then don't change from
            // the defaults declared above.
            if let Some(sub_attrs) = sub_attrs {
                // Iterate over all sub-attributes
                for sub_attr in sub_attrs.iter() {
                    if let Some((ident, val)) = sub_attr!(sub_attr -> [message = str]?)? {
                        success.message = Some(Attribute::new(ident.clone(), val.1));
                    } else if let Some((ident, val)) = sub_attr!(sub_attr -> [json is potential])? {
                        success.json = Some(Attribute::new(ident.clone(), val));
                    } else {
                        return err!(sub_attr, "Unknown sub-attribute");
                    }
                }
            }

            success_failure = Some(SuccessFailure::Success(success));
        } else if let Some((ident, sub_attrs)) = attr!(attr == #[failure(..)?]).found() {
            // Check if a success or failure attribute already exists
            if success_failure.is_some() {
                return err!(ident, "Only one success or failure attribute is allowed");
            }

            // Create a default Failure struct.
            let mut failure = Failure {
                ident: ident.clone(),
                message: None,
                json: None,
            };

            // Check that the Failure attribute has sub-attributes
            // If it doesnt (i.e., just #[failure]) then don't change from
            // the defaults declared above.
            if let Some(sub_attrs) = sub_attrs {
                // Iterate over all sub-attributes
                for sub_attr in sub_attrs.iter() {
                    if let Some((ident, val)) = sub_attr!(sub_attr -> [message = str]?)? {
                        failure.message = Some(Attribute::new(ident.clone(), val.1));
                    } else if let Some((ident, val)) = sub_attr!(sub_attr -> [json is potential])? {
                        failure.json = Some(Attribute::new(ident.clone(), val));
                    } else {
                        return err!(sub_attr, "Unknown sub-attribute");
                    }
                }
            }

            success_failure = Some(SuccessFailure::Failure(failure));
        } else if let Some(status_code) = attr!(attr == #[status_code(int!)])? {
            // literally just to make typing of `status_code` more clear
            // to myself and anyone reading this.
            // status_code.0 is the Ident of the attribute
            // status_code.1 is the Ident and Int of the value of the attribute
            let status_code: (&syn::Ident, (proc_macro2::Span, u16)) = status_code;
            status = Some((status_code.1 .0, status_code.1 .1));
        }
    }

    let success_failure = success_failure
        .ok_or_else(|| syn::Error::new(unfound_span, "No success or failure attribute found"))?;

    Ok((status, success_failure))
}
