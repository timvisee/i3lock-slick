extern crate yaml_rust;

use std::collections::{BTreeMap};

use self::yaml_rust::{Yaml, YamlLoader};

use err::{Error, Result};

/// A trait providing helper functions for a `Yaml` object.
/// These helper functions make it easy to get and set properties in such object.
///
///
pub trait YamlHelper {
    /// Get a Yaml property from this Yaml object at the given `node`.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, the object itself is returned.
    ///
    /// If the property is not found, `None` is returned.
    fn property<'a: 'b, 'b>(&'a self, node: &'b str) -> Option<&'b Yaml>;

    /// Get a Yaml property from the given Yaml object at the given `node`.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, the `object` itself is returned.
    ///
    /// If the property is not found, `None` is returned.
    fn property_from<'a: 'b, 'b>(object: &'a Yaml, node: &str) -> Option<&'b Yaml>;

    /// Set a Yaml property in the current object.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, this object itself it set to the given value.
    ///
    /// If the property isn't found, maps are created automatically to get the proper result structure.
    fn set_property<'a>(&mut self, node: &str, value: Yaml) -> Result<'a, ()>;

    /// Set a Yaml property at the given `node`.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, the `object` itself it set to the given value.
    ///
    /// If the property isn't found, maps are created automatically to get the proper result structure.
    fn set_property_in<'a>(object: &mut Yaml, node: &str, value: Yaml) -> Result<'a, ()>;

    /// Check whether the `node` is in this Yaml object.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, this object is evaluated.
    ///
    /// If the property is found, but it is `Null`, `false` is also returned.
    fn has_property(&self, node: &str) -> bool;

    /// Check whether the `node` is in the given Yaml `object`.
    ///
    /// Keys inside the `node` may be separated by a `.`.
    ///
    /// If `node` is an empty string, the `object` is evaluated.
    ///
    /// If the property is found, but it is `Null`, `false` is also returned.
    fn has_property_in(object: &Yaml, node: &str) -> bool;
}

// TODO: Just put these functions in the main trait, leave this implementation empty
/// Implement additional YAML features.
impl YamlHelper for Yaml {
    fn property<'a: 'b, 'b>(&'a self, node: &'b str) -> Option<&'b Yaml> {
        Yaml::property_from(self, node)
    }

    fn property_from<'a: 'b, 'b>(object: &'a Yaml, node: &str) -> Option<&'b Yaml> {
        let node = node.trim();

        // Return the object is the node is empty
        if node.is_empty() {
            return Some(object);
        }

        // Get the YAML object as map
        if let Some(map) = object.as_hash() {
            // A list of keys
            let mut keys = node.splitn(2, '.');

            // Get the Yaml key
            let key = YamlLoader::load_from_str(keys.next().unwrap());
            if key.is_err() {
                return None;
            }

            // Get the value for the first key
            match map.get(&key.unwrap()[0]) {
                Some(value) =>
                // Handle the rest of the node
                    Yaml::property_from(&value, keys.next().unwrap_or("")),
                None => None,
            }

        } else {
            None
        }
    }

    fn set_property<'a>(&mut self, node: &str, value: Yaml) -> Result<'a, ()> {
        Yaml::set_property_in(self, node, value)
    }

    fn set_property_in<'a>(object: &mut Yaml, node: &str, value: Yaml) -> Result<'a, ()> {
        let node = node.trim();

        // Return the object is the node is empty
        if node.is_empty() {
            *object = value;
            return Ok(());
        }

        // A list of keys
        let mut keys = node.splitn(2, '.');

        // Get the Yaml key
        let key = Yaml::from_str(keys.next().unwrap());

        match *object {
            Yaml::Hash(ref mut map) => {
                // Get the value for the first key
                let mut object: &mut Yaml = match map.get_mut(&key) {
                    Some(value) => value,
                    None => {
                        map.insert(key.clone(), Yaml::Hash(BTreeMap::new()));
                        *map.get_mut(&key).unwrap()
                    }
                };

                Yaml::set_property_in(&mut object, keys.next().unwrap_or(""), value)
            },

            _ => {
                // Create new map, and set it as current object
                let mut map: BTreeMap<Yaml, Yaml> = BTreeMap::new();
                map.insert(key.clone(), Yaml::Null);
                *object = Yaml::Hash(map);

                Yaml::set_property_in(object, keys.next().unwrap_or(""), value)
            }
        }
    }

    fn has_property(&self, node: &str) -> bool {
        Yaml::has_property_in(self, node)
    }

    fn has_property_in(object: &Yaml, node: &str) -> bool {
        match object.property(node) {
            Some(value) =>
                match *value {
                    Yaml::Null => false,
                    _ => true,
                },
            None => false,
        }
    }
}
