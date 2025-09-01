use std::{marker::PhantomData, str::FromStr};

use snafu::Snafu;

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum UnitError {
    #[snafu(display("Unknown unit '{unit}'"))]
    Unknown { unit: String },
}

pub trait Unit: Copy + Default + FromStr + ToString {
    fn factor(&self) -> f32;
}

/// A unit type representing the absence of a unit, parameterized by `T` to allow type distinction.
/// The generic parameter `T` is used to differentiate between different unitless quantities at the type level.
///
/// # FromStr
///
/// The `from_str` implementation treats an empty string (`""`) as a valid representation of a unitless value.
/// Any other string will result in an error.
#[derive(Debug, PartialEq, Eq)]
pub struct Unitless<T>(PhantomData<T>);

impl<T> Copy for Unitless<T> {}

impl<T> Clone for Unitless<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Unitless<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T> Unit for Unitless<T> {
    fn factor(&self) -> f32 {
        1.
    }
}

impl<T> ToString for Unitless<T> {
    /// Returns an empty string to represent the "unitless" unit.
    fn to_string(&self) -> String {
        String::default()
    }
}

impl<T> FromStr for Unitless<T> {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Unitless::default()),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unitless_from_str() {
        assert_eq!(Unitless::<()>::from_str(""), Ok(Unitless::default()));
        assert_eq!(
            Unitless::<()>::from_str("unknown"),
            Err(UnitError::Unknown {
                unit: "unknown".to_string()
            })
        );
    }
}
