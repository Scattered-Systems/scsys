/*
    Appellation: display <module>
    Contrib: @FL03
*/
use syn::Ident;

#[derive(Clone, Debug, Default)]
pub struct DisplayAttr {
    pub serde: Option<Ident>,
}
