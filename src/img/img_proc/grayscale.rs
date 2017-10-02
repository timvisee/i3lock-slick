use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::ImgProc;

/// Image gray scale processor.
pub struct Grayscale {
    properties: HashMap<&'static str, String>
}

impl ImgProc for Grayscale {
    fn new() -> Self {
        Grayscale {
            properties: HashMap::new()
        }
    }

    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        Ok(ImgEdit::from(
            img.into_img().grayscale()
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, String> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, String> {
        &mut self.properties
    }
}