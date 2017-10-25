use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = HashMap::new();
}

/// Image gray scale processor.
pub struct Grayscale {
    properties: HashMap<&'static str, Prop>
}

impl Grayscale {
    pub fn new() -> Grayscale {
        Grayscale {
            properties: PROPERTIES.clone(),
        }
    }
}

impl ImgProc for Grayscale {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        Ok(ImgEdit::from(
            img.into_img().grayscale()
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}