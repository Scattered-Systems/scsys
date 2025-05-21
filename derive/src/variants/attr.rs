/*
    Appellation: display <module>
    Contrib: @FL03
*/
use syn::Ident;

#[derive(Default)]
pub struct DisplayAttr {
    pub kind: Option<syn::LitStr>,
    pub serde: Option<Ident>,
}

#[allow(dead_code)]
impl DisplayAttr {
    pub fn new() -> Self {
        Self {
            kind: None,
            serde: None,
        }
    }

    pub fn from_kind(kind: syn::LitStr) -> Self {
        Self {
            kind: Some(kind),
            ..Self::new()
        }
    }

    pub fn from_serde(serde: Ident) -> Self {
        Self {
            serde: Some(serde),
            ..Self::new()
        }
    }

    pub fn with_kind(self, kind: syn::LitStr) -> Self {
        Self {
            kind: Some(kind),
            ..self
        }
    }
    pub fn with_serde(self, serde: Ident) -> Self {
        Self {
            serde: Some(serde),
            ..self
        }
    }

    pub fn set_kind(&mut self, kind: syn::LitStr) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    pub fn set_serde(&mut self, serde: Ident) -> &mut Self {
        self.serde = Some(serde);
        self
    }
}

impl syn::parse::Parse for DisplayAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut display_attr = DisplayAttr::new();
        while !input.is_empty() {
            let attr: syn::Ident = input.parse()?;
            match attr.to_string().as_str() {
                "kind" => {
                    let kind: syn::LitStr = input.parse()?;
                    display_attr.set_kind(kind);
                }
                "serde" => {
                    let serde: Ident = input.parse()?;
                    display_attr.set_serde(serde);
                }
                _ => return Err(syn::Error::new(attr.span(), "Unknown attribute")),
            }
        }
        Ok(display_attr)
    }
}
