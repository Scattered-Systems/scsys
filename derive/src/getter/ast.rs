#![allow(dead_code)]

/// the abstract syntax tree (ast) for the `getter` macro
///
/// #[getter!(pub struct Foo { pub bar: u32 })]
/// generates:
/// ```no_run
/// pub struct Foo {
///
#[derive(Clone)]
pub struct GetterAst {
    pub vis: syn::Visibility,
    ///
    pub name: syn::Ident,
    pub field: syn::Ident,
    pub where_clause: Option<syn::WhereClause>,
    pub ty: syn::Type,
}

impl syn::parse::Parse for GetterAst {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse the visibility
        let vis = input.parse().unwrap_or(syn::Visibility::Inherited);
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
        let _: syn::Token![->] = input.parse()?;
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
