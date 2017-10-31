use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Property names
pub static PROP_SIGMA: &'static str = "sigma";
pub static PROP_THRESHOLD: &'static str = "sigma";

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = {
        let mut map = HashMap::new();
        map.insert(PROP_SIGMA, Prop::Float(None));
        map.insert(PROP_THRESHOLD, Prop::Int(None));
        map
    };
}

/// Image unsharpen processor.
pub struct Unsharpen {
    properties: HashMap<&'static str, Prop>
}

impl Unsharpen {
    pub fn new() -> Unsharpen {
        Unsharpen {
            properties: PROPERTIES.clone()
        }
    }
}

impl ImgProc for Unsharpen {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(
            img.into_img()
                .unsharpen(
                    self.property(PROP_SIGMA).unwrap().as_float().unwrap(),
                    self.property(PROP_THRESHOLD).unwrap().as_int().unwrap(),
                )
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}