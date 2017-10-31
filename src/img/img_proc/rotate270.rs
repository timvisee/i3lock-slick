use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = HashMap::new();
}

/// Image 270 degrees rotate processor.
pub struct Rotate270 {
    properties: HashMap<&'static str, Prop>
}

impl Rotate270 {
    pub fn new() -> Rotate270 {
        Rotate270 {
            properties: PROPERTIES.clone(),
        }
    }
}

impl ImgProc for Rotate270 {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // Get the dynamic image and rotate
        let mut dyn_img = img.into_img();
        dyn_img.rotate270();

        Ok(ImgEdit::from(dyn_img))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}