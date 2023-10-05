use darling::FromVariant;
use proc_macro2::Ident;
use syn::{DataEnum, DeriveInput};

pub struct Enum {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(modes))]
pub struct Variant {
    pub ident: syn::Ident,
    #[darling(multiple, rename = "mode")]
    pub modes: Vec<String>,
}

impl Enum {
    pub fn from_syn(node: &DeriveInput, data: &DataEnum) -> syn::Result<Self> {
        let variants = data
            .variants
            .iter()
            .map(|node| {
                FromVariant::from_variant(node).expect("Should be able to parse the variants!")
            })
            .collect::<Vec<_>>();
        Ok(Enum {
            ident: node.ident.clone(),
            variants,
        })
    }
}
