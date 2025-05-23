use crate::attrs::{DisplayAttr, NestedAttr, VariantAttr};
use syn::Attribute;

// AST for the scsys attribute
#[derive(Debug, Default)]
pub struct ScsysAttr {
    pub display: Option<DisplayAttr>,
    pub variant: Option<VariantAttr>,
}

impl ScsysAttr {
    pub fn set_display(&mut self, display: DisplayAttr) {
        self.display = Some(display);
    }

    pub fn set_variant(&mut self, variant: VariantAttr) {
        self.variant = Some(variant);
    }

    // tries to extract the scsys attribute from a list of attributes
    pub fn extract(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut scsys = Self::default();
        for attr in attrs {
            if attr.path().is_ident("scsys") {
                attr.parse_nested_meta(|meta| {
                    if let Ok(nested) = NestedAttr::parse_nested(&meta) {
                        match nested {
                            NestedAttr::Display(inner) => {
                                scsys.set_display(inner);
                                return Ok(());
                            }
                            NestedAttr::Variant(inner) => {
                                scsys.set_variant(inner);
                                return Ok(());
                            }
                        }
                    }
                    Err(meta.error("unrecognized scsys attribute"))
                })?;
            }
        }
        Ok(scsys)
    }
}
