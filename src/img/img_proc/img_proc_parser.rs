extern crate regex;

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
    pub fn parse(filter: &str) -> Result<()> {
        // Define regular expressions for the full filter syntax, and filter parameters
        let re_filter = Regex::new(r"^\s*([a-zA-Z]+)\s*(:[a-zA-Z0-9=,\.\s]*)?$").unwrap();
        let re_prop = Regex::new(r"^\s*([a-zA-Z]+)\s*=\s*([a-zA-Z0-9\.]*)\s*$").unwrap();

        // Get the filter components
        let mut filter_components = re_filter
            .captures_iter(filter);
        let filter_name: &str = filter_components
            .next()
            .ok_or(Error::new("Incorrect filter format"))?
            .get(1)
            .ok_or(Error::new("Failed to parse filter name"))?
            .as_str();

        println!("Filter name: {:?}", filter_name);

        // Get the filter properties
        if let Some(component_properties) = filter_components.next() {
            // Get the component properties, strip the colon
            let component_properties = &component_properties
                .get(1)
                .ok_or(Error::new("Failed to fetch properties component from filter string"))?
                .as_str()[1..];

            // Split the properties by the comma
            for component_property in component_properties.split(',') {
                let mut property_components = re_prop
                    .captures_iter(component_property);

                // Get the property name
                let prop_name = property_components
                    .next()
                    .ok_or(Error::new("Missing property name"))?
                    .get(1)
                    .ok_or(Error::new("Failed to fetch property name"))?
                    .as_str();

                // Get the property value
                let prop_val = property_components
                    .next()
                    .ok_or(Error::new("Missing property value"))?
                    .get(1)
                    .ok_or(Error::new("Failed to fetch property value"))?
                    .as_str();

                println!("Prop: {:?}, value: {:?}", prop_name, prop_val);
            }
        }

        // TODO: Return the filter here
        Ok(())
    }
}


