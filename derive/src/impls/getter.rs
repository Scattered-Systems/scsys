/*
    appellation: getter <module>
    authors: @FL03
*/
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::spanned::Spanned;

pub fn impl_getter(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Getter can only be derived for structs"),
    };

    let getters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub const fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#getters)*
        }
    }
}

pub fn impl_set(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Set can only be derived for structs"),
    };

    let setters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        let setter_name = syn::Ident::new(
            &format!("set_{}", field_name.as_ref().unwrap()),
            field_name.span(),
        );

        quote! {
            pub fn #setter_name(&mut self, value: #field_type) {
                self.#field_name = value;
            }
        }
    });

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#setters)*
        }
    }
}

pub fn impl_with(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("With can only be derived for structs"),
    };

    let with_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        let with_name = syn::Ident::new(
            &format!("with_{}", field_name.as_ref().unwrap()),
            field_name.span(),
        );

        quote! {
            pub fn #with_name(mut self, value: #field_type) -> Self {
                Self { #field_name: value, ..self }
            }
        }
    });

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#with_methods)*
        }
    }
}

pub fn _inject_generics(generics: &syn::Generics) -> syn::Generics {
    // Clone the generics and add `Clone` bounds if necessary
    let mut generics = generics.clone();
    for param in generics.type_params_mut() {
        param.bounds.push(syn::parse_quote!(Clone));
    }
    generics
}
