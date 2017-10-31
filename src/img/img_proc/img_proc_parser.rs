extern crate regex;

use super::*;

use self::regex::Regex;

use err::{Error, Result};

/// Image processor parser.
///
/// This parses an image processor with options from a given string.
/// This is useful to filters based on user input from the command line.
pub struct ImgProcParser {}

impl ImgProcParser {

    /// Parse the filter form the given `filter` string.
    ///
    /// An error is returned if parsing failed.
    pub fn parse<'a: 'b, 'b>(filter: &'b str) -> Result<'a, Box<ImgProc>> {
        // Define regular expressions for the full filter syntax, and filter parameters
        let re_filter = Regex::new(r"^\s*([a-zA-Z]+)\s*(:[a-zA-Z0-9=;,\.\-\s]*)?$").unwrap();
        let re_prop = Regex::new(r"^\s*([a-zA-Z]+)\s*=\s*([a-zA-Z0-9\.\-]*)\s*$").unwrap();

        // Get the filter components, skip the first super match
        let filter_matches = re_filter
            .captures_iter(filter)
            .next()
            .ok_or(Error::new("Incorrect filter format"))?;
        let mut filter_components = filter_matches.iter();
        filter_components.next();

        // Determine the filter name
        let filter_name: &str = filter_components
            .next()
            .ok_or(Error::new("Unable to find filter name, maybe it is missing"))?
            .ok_or(Error::new("Failed to parse the filter name"))?
            .as_str();

        println!("Filter name: {:?}", filter_name);

        // Create a list of property strings
        let mut property_strings: Vec<(&str, &str)> = vec![];

        // Get the filter properties if there are any
        if let Some(Some(component_properties)) = filter_components.next() {
            // Get the component properties, strip the colon
            let component_properties = &component_properties.as_str()[1..];

            // Split the properties by the comma
            for component_property in component_properties.split(';') {
                // Get the property components, skip the first super match
                let property_matches = re_prop
                    .captures_iter(component_property)
                    .next()
                    .ok_or(Error::new("Incorrect filter property format"))?;
                let mut property_components = property_matches.iter();
                property_components.next();

                // Get the property name
                let prop_name = property_components
                    .next()
                    .ok_or(Error::new("Missing filter property name"))?
                    .ok_or(Error::new("Unable to parse filter property name"))?
                    .as_str();

                // Get the property value
                let prop_val = property_components
                    .next()
                    .ok_or(Error::new("Missing filter property value"))?
                    .ok_or(Error::new("Unable to parse filter property value"))?
                    .as_str();

                println!("Prop: {:?}, value: {:?}", prop_name, prop_val);

                // Add the property name and value to the property strings list
                property_strings.push((prop_name, prop_val));
            }
        }

        // Parse the filter and filter property strings, return the result
        ImgProcParser::parse_parts(filter_name, property_strings)
    }

    /// Parse the filter with the given `name`, and set the given `properties` on it.
    ///
    /// If the filter name is unknown, an error is returned.
    ///
    /// The properties are in `key`-`value` tuple format.
    /// Properties that are not known for the filter return an error.
    pub fn parse_parts<'a>(name: &str, properties: Vec<(&str, &str)>) -> Result<'a, Box<ImgProc>> {
        // Parse the filter by it's name
        let mut filter = ImgProcParser::create_filter_by_name(name)?;

        // Apply each filter
        for (key, value) in properties {
            filter.set_property(key, value)?;
        }

        Ok(filter)
    }

    /// Create a filter instance by the given filter `name`.
    ///
    /// If the name `blur` is given, a new instance of the image blur processor is returned.
    ///
    /// An error is returned if the filter name is unknown.
    pub fn create_filter_by_name<'a: 'b, 'b>(name: &'b str) -> Result<'a, Box<ImgProc>> {
        match name.trim().to_lowercase() {
            "blur" => Ok(Box::new(Blur::new())),
            "brighten" => Ok(Box::new(Brighten::new())),
            "contrast" => Ok(Box::new(Contrast::new())),
            "greyscale" => Ok(Box::new(Greyscale::new())),
            "huerotate" => Ok(Box::new(HueRotate::new())),
            "invert" => Ok(Box::new(Invert::new())),
            _ => Err(Error::new("Unknown filter name")),
        }
    }
}