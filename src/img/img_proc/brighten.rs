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
        map.insert(PROP_AMOUNT, Prop::Int(None));
        map
    };
}

/// Image brighten processor.
pub struct Brighten {
    properties: HashMap<&'static str, Prop>
}

impl Brighten {
    pub fn new() -> Brighten {
        Brighten {
            properties: PROPERTIES.clone()
        }
    }
}

impl ImgProc for Brighten {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // TODO: Handle errors!
        Ok(ImgEdit::from(
            img.into_img()
                .brighten(self.property(PROP_AMOUNT).unwrap().as_int().unwrap())
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}