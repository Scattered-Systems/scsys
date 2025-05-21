/*
    Appellation: attrs <module>
    Contrib: @FL03
*/
#![allow(dead_code)]

use crate::{display::DisplayAttr, params::ParamsAttr};

#[derive(Default)]
pub struct ScsysAttr {
    pub display: Option<DisplayAttr>,
    pub params: Option<ParamsAttr>,
}

impl ScsysAttr {
    pub fn new() -> Self {
        Self {
            display: None,
            params: None,
        }
    }

    pub fn from_display(display: DisplayAttr) -> Self {
        Self {
            display: Some(display),
            ..Self::new()
        }
    }

    pub fn from_params(params: ParamsAttr) -> Self {
        Self {
            params: Some(params),
            ..Self::new()
        }
    }

    pub fn set_display(&mut self, display: DisplayAttr) -> &mut Self {
        self.display = Some(display);
        self
    }
    pub fn set_params(&mut self, params: ParamsAttr) -> &mut Self {
        self.params = Some(params);
        self
    }
    /// returns true if the display attribute is _**not**_ present
    pub fn is_display_none(&self) -> bool {
        self.display.is_none()
    }
    /// returns true if the display attribute is present
    pub fn is_display_some(&self) -> bool {
        self.display.is_some()
    }
    /// returns true if the params attribute is _**not**_ present
    pub fn is_params_none(&self) -> bool {
        self.params.is_none()
    }
    /// returns true if the params attribute is present
    pub fn is_params_some(&self) -> bool {
        self.params.is_some()
    }
}

impl syn::parse::Parse for ScsysAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut _attr = ScsysAttr::new();
        if let Ok(arg) = input.parse::<DisplayAttr>() {
            _attr.set_display(arg);
        }
        if let Ok(arg) = input.parse::<ParamsAttr>() {
            _attr.set_params(arg);
        }

        Ok(_attr)
    }
}

use syn::{Ident, parenthesized};

pub fn _handle_display_attr(attrs: &Vec<syn::Attribute>) -> syn::Result<ScsysAttr> {
    let mut _attr = ScsysAttr::new();

    for attr in attrs {
        if attr.path().is_ident("scsys") {
            attr.parse_nested_meta(|meta| {
                // #[scsys(display(...))]
                if meta.path.is_ident("display") {
                    let content;
                    parenthesized!(content in meta.input);
                    let lit: syn::LitStr = content.parse()?;
                    let opt: Ident = content.parse()?;
                    _attr.set_display(DisplayAttr::from_kind(lit).with_serde(opt));
                }

                // #[scsys(params(...))]
                if meta.path.is_ident("params") {
                    return Ok(());
                }

                Err(meta.error("unrecognized repr"))
            })?;
        }
    }
    Ok(_attr)
}
