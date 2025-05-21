/*
    Appellation: attrs <module>
    Contrib: @FL03
*/
#![allow(dead_code)]
#[doc(inline)]
pub use self::{display::DisplayAttr, params::ParamsAttr};

mod display;
mod params;

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
