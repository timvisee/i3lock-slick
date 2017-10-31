use std::collections::HashMap;

use err::Result;
use img::ImgEdit;

use super::{ImgProc, Prop};

// Property names
pub static PROP_AMOUNT: &'static str = "amount";

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = {
        let mut map = HashMap::new();
        map.insert(PROP_AMOUNT, Prop::Float(None));
        map
    };
}

/// Image contrast processor.
pub struct Contrast {
    properties: HashMap<&'static str, Prop>
}

impl Contrast {
    pub fn new() -> Contrast {
        Contrast {
            properties: PROPERTIES.clone()
        }
    }
}

impl ImgProc for Contrast {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(
            img.into_img()
                .adjust_contrast(self.property(PROP_AMOUNT).unwrap().as_float().unwrap())
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}