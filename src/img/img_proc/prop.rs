use err::Result;

#[derive(Clone)]
pub enum Prop {
    /// A float value.
    Float(Option<f32>),
}

impl Prop {
    /// Parse the given `value` and try to fit it in the current type.
    ///
    /// If parsing fails or if the given `value` is not compatible with the type,
    /// an error is returned.
    pub fn parse<'a>(&mut self, value: &str) -> Result<'a, ()> {
        // Parse and update the type
        match *self {
            Prop::Float(ref mut x) => {
                *x = Some(value.parse::<f32>()?);
                Ok(())
            }
        }
    }

    /// Check whether the property is empty.
    pub fn is_empty(&self) -> bool {
        match *self {
            Prop::Float(x) => x.is_none(),
        }
    }

    /// Get the property as float.
    pub fn as_float(&self) -> Option<f32> {
        match *self {
            Prop::Float(x) => x,
        }
    }
}