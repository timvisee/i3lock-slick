use std::collections::HashMap;

use img::ImgEdit;
use err::{Error, Result};

/// Image processor trait.
pub trait ImgProc {
    /// Constructor with defaults.
    fn new() -> Self;

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
    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, String>;

    /// List of mutable properties.
    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, String>;

    /// Get a list of empty properties.
    fn empty_properties(&self) -> Vec<&'static str> {
        self.properties()
            .iter()
            .filter(|property| property.1.trim().is_empty())
            .map(|property| *property.0)
            .collect()
    }

    /// Get a reference to a property by `name`.
    fn property<'a: 'b, 'b>(&'a self, name: &'b str) -> Option<&'b String> {
        self.properties().get(name)
    }

    /// Get a mutable reference to a property by `name`.
    fn mut_property<'a: 'b, 'b>(&'a mut self, name: &'b str) -> Option<&'b mut String> {
        self.mut_properties().get_mut(name)
    }

    /// Set a property by the given `name` to the given `value`.
    ///
    /// Returns an error if the property wasn't found, or if the value was invalid.
    fn set_property<'a>(&'a mut self, name: &'static str, value: String) {
        *self.mut_properties()
            .entry(&name)
            .or_insert(String::new()) = value
    }
}

