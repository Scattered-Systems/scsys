/*
    Appellation: params <module>
    Contrib: @FL03
*/

use syn::Ident;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VariantAttr {
    pub name: Option<Ident>,
}

impl VariantAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let content: syn::parse::ParseBuffer<'_>;
        syn::parenthesized!(content in meta.input);
        // try finding the optional name parameter
        let name = content.parse::<Ident>();
        // create a new instance of ParamsAttr
        let parsed = VariantAttr { name: name.ok() };
        // return the parsed instance
        Ok(parsed)
    }
}
