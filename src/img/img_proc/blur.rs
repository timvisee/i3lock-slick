use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Property names
pub static PROP_SIGMA: &'static str = "sigma";

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = {
        let mut map = HashMap::new();
        map.insert(PROP_SIGMA, Prop::Float(Some(5.0)));
        map
    };
}

/// Image blurring processor.
pub struct Blur {
    properties: HashMap<&'static str, Prop>
}

impl Blur {
    pub fn new() -> Blur {
        Blur {
            properties: PROPERTIES.clone()
        }
    }
}

impl ImgProc for Blur {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(
            img.into_img()
                .blur(self.property(PROP_SIGMA).unwrap().as_float().unwrap())
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}