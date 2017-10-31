use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Property names
pub static PROP_X: &'static str = "x";
pub static PROP_Y: &'static str = "y";
pub static PROP_WIDTH: &'static str = "width";
pub static PROP_HEIGHT: &'static str = "height";

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = {
        let mut map = HashMap::new();
        map.insert(PROP_X, Prop::UInt(None));
        map.insert(PROP_Y, Prop::UInt(None));
        map.insert(PROP_WIDTH, Prop::UInt(None));
        map.insert(PROP_HEIGHT, Prop::UInt(None));
        map
    };
}

/// Image crop processor.
pub struct Crop {
    properties: HashMap<&'static str, Prop>
}

impl Crop {
    pub fn new() -> Crop {
        Crop {
            properties: PROPERTIES.clone()
        }
    }
}

impl ImgProc for Crop {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(
            img.into_img()
                .crop(
                    self.property(PROP_X).unwrap().as_uint().unwrap(),
                    self.property(PROP_Y).unwrap().as_uint().unwrap(),
                    self.property(PROP_WIDTH).unwrap().as_uint().unwrap(),
                    self.property(PROP_HEIGHT).unwrap().as_uint().unwrap(),
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