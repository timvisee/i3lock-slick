use std::collections::HashMap;

use img::ImgEdit;
use err::{Error, Result};

use super::Prop;

/// Image processor trait.
pub trait ImgProc {
    /// Process the given image `img` in a safe way with the current processor.
    ///
    /// The given image is consumed, a new and edited image is returned on success.
    ///
    /// This method checks whether all parameters are filled in before processing.
    /// An error may be returned if processing failed, or if the given parameters were incorrect.
    fn process_safe(&self, img: ImgEdit) -> Result<ImgEdit> {
        if !self.empty_properties().is_empty() {
            Err(Error::new("Unable to start image processor, missing properties"))
        } else {
            self.process(img)
        }
    }

    /// Process the given image `img` with the current processor.
    ///
    /// The given image is consumed, a new and edited image is returned on success.
    ///
    /// An error may be returned if processing failed, or if the given parameters were incorrect.
    fn process(&self, img: ImgEdit) -> Result<ImgEdit>;

    /// List of properties.
    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop>;

    /// List of mutable properties.
    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop>;

    /// Get a list of empty properties.
    fn empty_properties(&self) -> Vec<&'static str> {
        self.properties()
            .iter()
            .filter(|property| property.1.is_empty())
            .map(|property| *property.0)
            .collect()
    }

    /// Get a reference to a property by `name`.
    fn property<'a: 'b, 'b>(&'a self, name: &'b str) -> Option<&'b Prop> {
        self.properties().get(name)
    }

    /// Get a mutable reference to a property by `name`.
    fn mut_property<'a: 'b, 'b>(&'a mut self, name: &'b str) -> Option<&'b mut Prop> {
        self.mut_properties().get_mut(name)
    }

    /// Set a property by the given `name` to the given string `value`.
    ///
    /// Returns an error if the property wasn't found,
    /// or it the value couldn't be parsed because it was incompatible.
    fn set_property<'a>(&mut self, name: &str, value: &str) -> Result<'a, ()> {
        match self.mut_property(name) {
            Some(ref mut prop) => prop.parse(value.as_ref()),
            None => Err(Error::new("Tried to set unknown option on a filter")),
        }
    }
}
