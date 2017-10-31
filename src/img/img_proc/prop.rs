use err::Result;

#[derive(Clone)]
pub enum Prop {
    /// An integer value.
    Int(Option<i32>),

    /// An unsigned integer value.
    UInt(Option<u32>),

    /// A float value.
    Float(Option<f32>),

    /// A boolean value.
    Bool(Option<bool>),

    /// A string value.
    String(Option<String>),
}

impl Prop {
    /// Parse the given `value` and try to fit it in the current type.
    ///
    /// If parsing fails or if the given `value` is not compatible with the type,
    /// an error is returned.
    pub fn parse<'a>(&mut self, value: &str) -> Result<'a, ()> {
        // Parse and update the type
        match *self {
            Prop::Int(ref mut x) => {
                *x = Some(value.parse::<i32>()?);
            },
            Prop::UInt(ref mut x) => {
                *x = Some(value.parse::<u32>()?);
            },
            Prop::Float(ref mut x) => {
                *x = Some(value.parse::<f32>()?);
            }

            // TODO: parse a boolean value
            Prop::Bool(ref mut x) => {
                *x = Some(value.parse::<bool>().unwrap_or(false));
            },

            Prop::String(ref mut x) => {
                *x = Some(value.to_string());
            },
        }

        Ok(())
    }

    /// Check whether the property is empty.
    pub fn is_empty(&self) -> bool {
        match *self {
            Prop::Int(x) => x.is_none(),
            Prop::UInt(x) => x.is_none(),
            Prop::Float(x) => x.is_none(),
            Prop::Bool(x) => x.is_none(),
            Prop::String(ref x) => x.clone().map(|x| x.is_empty()).unwrap_or(true),
        }
    }

    /// Get the property as integer.
    pub fn as_int(&self) -> Option<i32> {
        match *self {
            Prop::Int(x) => x,
            Prop::UInt(x) => x.map(|x| x as i32),
            Prop::Float(x) => x.map(|x| x as i32),
            Prop::Bool(x) => x.map(|x| if x { 1 } else { 0 }),
            Prop::String(ref x) => x.clone()
                .map(|x| x.parse::<i32>().ok())
                .unwrap_or(None),
        }
    }

    /// Get the property as unsigned integer.
    pub fn as_uint(&self) -> Option<u32> {
        match *self {
            Prop::Int(x) => x.map(|x| x as u32),
            Prop::UInt(x) => x,
            Prop::Float(x) => x.map(|x| x as u32),
            Prop::Bool(x) => x.map(|x| if x { 1 } else { 0 }),
            Prop::String(ref x) => x.clone()
                .map(|x| x.parse::<u32>().ok())
                .unwrap_or(None),
        }
    }

    /// Get the property as float.
    pub fn as_float(&self) -> Option<f32> {
        match *self {
            Prop::Int(x) => x.map(|x| x as f32),
            Prop::UInt(x) => x.map(|x| x as f32),
            Prop::Float(x) => x,
            Prop::Bool(x) => x.map(|x| if x { 1f32 } else { 0f32 }),
            Prop::String(ref x) => x.clone()
                .map(|x| x.parse::<f32>().ok())
                .unwrap_or(None),
        }
    }

    /// Get the property as boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Prop::Int(x) => x.map(|x| x != 0),
            Prop::UInt(x) => x.map(|x| x != 0),
            Prop::Float(x) => x.map(|x| x != 0f32),
            Prop::Bool(x) => x,

            // TODO: parse a boolean value
            Prop::String(ref x) => x.clone()
                .map(|x| x.parse::<bool>().ok())
                .unwrap_or(None),
        }

    }

    /// Get the property as string.
    pub fn as_str(&self) -> Option<String> {
        match *self {
            Prop::Int(x) => x.map(|x| x.to_string()),
            Prop::UInt(x) => x.map(|x| x.to_string()),
            Prop::Float(x) => x.map(|x| x.to_string()),
            Prop::Bool(x) => x.map(|x| if x { "true" } else { "false" }).map(|x| x.into()),
            Prop::String(ref x) => x.clone(),
        }
    }
}