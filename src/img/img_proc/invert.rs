use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::ImgProc;

/// Image gray scale processor.
pub struct Invert {
    properties: HashMap<&'static str, String>
}

impl ImgProc for Invert {
    fn new() -> Self {
        Invert {
            properties: HashMap::new()
        }
    }

    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        Ok(ImgEdit::from(
            img.into_img().invert()
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, String> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, String> {
        &mut self.properties
    }
}