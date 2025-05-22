#![allow(dead_code)]

use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token, Visibility, WhereClause};

/// the abstract syntax tree (ast) for the `getter` macro
///
/// #[getter!(pub struct Foo { pub bar: u32 })]
/// generates:
/// ```no_run
/// pub struct Foo {
///
#[derive(Clone)]
pub struct GetterAst {
    pub vis: Visibility,
    ///
    pub name: Ident,
    pub field: Ident,
    pub where_clause: Option<WhereClause>,
    pub ty: syn::Type,
}

impl Parse for GetterAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parse the visibility
        let vis = input.parse().unwrap_or(Visibility::Inherited);
        // parse the struct name
        let name = input.parse()?;
        // parse the field name
        let field = input.parse()?;
        // parse the field name
        let where_clause = if input.peek(syn::token::Where) {
            input.parse().ok()
        } else {
            None
        };
        let _: Token![->] = input.parse()?;
        let ty = input.parse()?;

        let tree = GetterAst {
            vis,
            name,
            field,
            ty,
            where_clause,
        };
        Ok(tree)
    }
}
