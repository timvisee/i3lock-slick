use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::ImgProc;

// Property names
pub static PROP_SIGMA: &'static str = "sigma";

/// Image blurring processor.
pub struct Blur {
    properties: HashMap<&'static str, String>
}

impl ImgProc for Blur {
    fn new() -> Self {
        // Build the list with properties
        let mut properties: HashMap<&'static str, String> = HashMap::new();
        properties.insert(PROP_SIGMA, "3".into());

        Blur {
            properties
        }
    }

    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(img.into_img().blur(self.property(PROP_SIGMA).unwrap().parse().unwrap())))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, String> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, String> {
        &mut self.properties
    }
}