use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = HashMap::new();
}

/// Image gray scale processor.
pub struct Invert {
    properties: HashMap<&'static str, Prop>
}

impl ImgProc for Invert {
    fn new() -> Self {
        Invert {
            properties: PROPERTIES.clone(),
        }
    }

    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // Get the dynamic image and invert
        let mut dyn_img = img.into_img();
        dyn_img.invert();

        Ok(ImgEdit::from(dyn_img))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}