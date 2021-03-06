use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = HashMap::new();
}

/// Image 90 degrees rotate processor.
pub struct Rotate90 {
    properties: HashMap<&'static str, Prop>
}

impl Rotate90 {
    pub fn new() -> Rotate90 {
        Rotate90 {
            properties: PROPERTIES.clone(),
        }
    }
}

impl ImgProc for Rotate90 {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        Ok(ImgEdit::from(
            img.into_img().rotate90()
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}