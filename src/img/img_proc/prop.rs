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

            // TODO: Parse a boolean
            Prop::Bool(ref mut x) => {
                *x = Some(value.parse::<bool>()?);
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
        }
    }

    /// Get the property as integer.
    pub fn as_int(&self) -> Option<i32> {
        match *self {
            Prop::Int(x) => x,
            Prop::Uint(x) => x.map(|x| x as i32),
            Prop::Float(x) => x.map(|x| x as i32),
            Prop::Bool(x) => x.map(|x| if x { 1 } else { 0 })
        }
    }

    /// Get the property as unsigned integer.
    pub fn as_uint(&self) -> Option<u32> {
        match *self {
            Prop::Int(x) => x.map(|x| x as u32)
            Prop::Uint(x) => x,
            Prop::Float(x) => x.map(|x| x as u32),
            Prop::Bool(x) => x.map(|x| if x { 1 } else { 0 })
        }
    }

    /// Get the property as float.
    pub fn as_float(&self) -> Option<f32> {
        match *self {
            Prop::Int(x) => x.map(|x| x as f32),
            Prop::UInt(x) => x.map(|x| x as f32),
            Prop::Float(x) => x,
            Prop::Bool(x) => x.map(|x| if x { 1f32 } else { 0f32 }),
        }
    }

    /// Get the property as boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Prop::Int(x) => x.map(|x| x != 0),
            Prop::UInt(x) => x.map(|x| x != 0),
            Prop::Float(x) => x.map(|x| x != 0),
            Prop::Bool(x) => x,
        }
    }
}