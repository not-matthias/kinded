use crate::models::{FieldsType, KindedAttributes, Meta, Variant};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Data, DeriveInput, Token,
};

pub fn parse_derive_input(input: DeriveInput) -> Result<Meta, syn::Error> {
    eprintln!("{input:#?}");

    let kinded_attrs: KindedAttributes = {
        match find_kinded_attr(&input)? {
            Some(kinded_attr) => syn::parse2(kinded_attr.to_token_stream())?,
            None => KindedAttributes::default(),
        }
    };

    let data = match input.data {
        Data::Enum(enum_data) => enum_data,
        Data::Struct(..) | Data::Union(..) => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Kinded can be derived only on enums",
            ));
        }
    };

    Ok(Meta {
        vis: input.vis,
        ident: input.ident,
        variants: data.variants.iter().map(parse_variant).collect(),
        kinded_attrs,
    })
}

fn parse_variant(variant: &syn::Variant) -> Variant {
    Variant {
        ident: variant.ident.clone(),
        fields_type: parse_fields_type(&variant.fields),
    }
}

fn parse_fields_type(fields: &syn::Fields) -> FieldsType {
    match fields {
        syn::Fields::Named(..) => FieldsType::Named,
        syn::Fields::Unnamed(..) => FieldsType::Unnamed,
        syn::Fields::Unit => FieldsType::Unit,
    }
}

/// Find `#[kinded(..)]` attribute on the enum.
fn find_kinded_attr(input: &DeriveInput) -> Result<Option<&Attribute>, syn::Error> {
    let kinded_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|&attr| attr.path().is_ident("kinded"))
        .collect();

    if kinded_attrs.len() > 1 {
        let &attr = kinded_attrs.last().unwrap();
        let span = attr.span();
        let msg = "Multiple #[kinded(..)] attributes are not allowed.";
        return Err(syn::Error::new(span, msg));
    } else {
        let maybe_kinded_attr = kinded_attrs.into_iter().next();
        Ok(maybe_kinded_attr)
    }
}

impl Parse for KindedAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut kinded_attrs = KindedAttributes::default();

        // Unwrap the irrelevant part and reassign input to the relevant input:
        //
        //     #[kinded(  RELEVANT_INPUT  )]
        //
        let input = {
            let _: Token!(#) = input.parse()?;
            let bracketed_content;
            bracketed!(bracketed_content in input);
            let _kinded: Ident = bracketed_content.parse()?;

            let parenthesized_content;
            parenthesized!(parenthesized_content in bracketed_content);
            parenthesized_content
        };

        let attr_name: Ident = input.parse()?;
        if attr_name == "kind" {
            let _: Token!(=) = input.parse()?;
            let kind: Ident = input.parse()?;
            if kinded_attrs.kind.is_none() {
                kinded_attrs.kind = Some(kind);
            } else {
                let msg = format!("Duplicated attribute: {attr_name}");
                return Err(syn::Error::new(attr_name.span(), msg));
            }
        } else {
            let msg = format!("Unknown attribute: {attr_name}");
            return Err(syn::Error::new(attr_name.span(), msg));
        }

        Ok(kinded_attrs)
    }
}