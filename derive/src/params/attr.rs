/*
    Appellation: params <module>
    Contrib: @FL03
*/
#![allow(dead_code)]
use syn::Ident;

#[derive(Default)]
pub struct ParamsAttr {
    pub name: Option<Ident>,
}

impl syn::parse::Parse for ParamsAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "name" {
                name = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(ident.span(), "unexpected parameter"));
            }
        }
        Ok(Self { name })
    }
}
