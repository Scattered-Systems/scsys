/*
    Appellation: params <module>
    Contrib: @FL03
*/
use syn::Ident;

#[derive(Clone, Debug, Default)]
pub struct ParamsAttr {
    pub name: Option<Ident>,
}
