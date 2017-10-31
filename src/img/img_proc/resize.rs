extern crate image;

use std::collections::HashMap;

use self::image::FilterType;

use err::{Error, Result};
use img::ImgEdit;

use super::{ImgProc, Prop};

// Property names
pub static PROP_WIDTH: &'static str = "width";
pub static PROP_HEIGHT: &'static str = "height";
pub static PROP_RATIO: &'static str = "ratio";
pub static PROP_FILTER: &'static str = "filter";

pub static FILTER_NEAREST: &'static str = "nearest";
pub static FILTER_NEAREST_SHORT: &'static str = "near";
pub static FILTER_TRIANGLE: &'static str = "triangle";
pub static FILTER_CATMULLROM: &'static str = "catmullrom";
pub static FILTER_GAUSSIAN: &'static str = "gaussian";
pub static FILTER_LANCZOS3: &'static str = "lanczos3";

// Default properties
lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, Prop> = {
        let mut map = HashMap::new();
        map.insert(PROP_WIDTH, Prop::UInt(None));
        map.insert(PROP_HEIGHT, Prop::UInt(None));
        map.insert(PROP_RATIO, Prop::Bool(Some(false)));
        map.insert(PROP_FILTER, Prop::String(Some(FILTER_TRIANGLE.into())));
        map
    };
}

/// Image resize processor.
pub struct Resize {
    properties: HashMap<&'static str, Prop>
}

impl Resize {
    pub fn new() -> Resize {
        Resize {
            properties: PROPERTIES.clone()
        }
    }

    /// Parse the given `filter` name into a `FilterType`.
    ///
    /// The filter name is case-insensitive and is trimmed.
    ///
    /// If the filter name was unknown, an error is returned.
    pub fn parse_filter<'a: 'b, 'b>(filter: &'b str) -> Result<'a, FilterType> {
        // Normalize the filter name
        let filter = filter.trim().to_lowercase();

        // Return the proper filter
        if &filter == FILTER_NEAREST || &filter == FILTER_NEAREST_SHORT {
            Ok(FilterType::Nearest)
        } else if &filter == FILTER_TRIANGLE {
            Ok(FilterType::Triangle)
        } else if &filter == FILTER_CATMULLROM {
            Ok(FilterType::CatmullRom)
        } else if &filter == FILTER_GAUSSIAN {
            Ok(FilterType::Gaussian)
        } else if &filter == FILTER_LANCZOS3 {
            Ok(FilterType::Lanczos3)
        } else {
            // No filter found, return an error
            Err(Error::new("Unknown filter name."))
        }
    }
}

impl ImgProc for Resize {
    fn process(&self, img: ImgEdit) -> Result<ImgEdit> {
        // Parse the filter to use
        let filter_name = self.property(PROP_FILTER).unwrap().as_str().unwrap();
        let filter = Resize::parse_filter(&filter_name)?;

        // Parse the width and height properties
        let width = self.property(PROP_WIDTH).unwrap().as_uint().unwrap();
        let height = self.property(PROP_HEIGHT).unwrap().as_uint().unwrap();

        // TODO: Handle errors!
        Ok(ImgEdit::from(
            if self.property(PROP_RATIO).unwrap().as_bool().unwrap() {
                img.into_img()
                    .resize(width, height, filter)
            } else {
                img.into_img()
                    .resize_exact(width, height, filter)
            }
        ))
    }

    fn properties<'a: 'b, 'b>(&'a self) -> &'b HashMap<&'static str, Prop> {
        &self.properties
    }

    fn mut_properties<'a: 'b, 'b>(&'a mut self) -> &'b mut HashMap<&'static str, Prop> {
        &mut self.properties
    }
}