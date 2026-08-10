use std::marker::PhantomData;

pub struct Second;

/// `Unit`, a trait indicating that a type can be used as a unit of measurement
/// and providing a method to get the conversion factor to a canonical unit.
pub trait Unit: Copy + Default + ToString {
    /// Returns the conversion factor to a canonical unit.
    fn factor(&self) -> f32;

    /// Indicates whether the unit is unitless.
    fn is_unitless() -> bool {
        false
    }
}

/// A unit type representing the absence of a unit, parameterized by `T` to allow type distinction
/// at type level.
#[derive(Debug)]
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

impl<T> PartialEq for Unitless<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Unitless<T> {}

impl<T> ToString for Unitless<T> {
    /// Returns an empty string to represent the "unitless" unit.
    fn to_string(&self) -> String {
        String::default()
    }
}
