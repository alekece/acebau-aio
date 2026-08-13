use std::{fmt, marker::PhantomData};

use snafu::Snafu;

use crate::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
#[snafu(display("unknown unit '{unit}'"))]
pub struct UnitParseError {
    unit: String,
}

impl UnitParseError {
    pub fn new(unit: impl Into<String>) -> Self {
        Self { unit: unit.into() }
    }
}

pub struct Second;

/// `Unit`, a trait indicating that a type can be used as a unit of measurement
/// and providing a method to get the conversion factor to a canonical unit.
pub trait Unit: Copy + Default + ToString {
    /// Stable name used to compose type names for integrations such as GraphQL.
    const NAME: &'static str;

    /// Returns the conversion factor to a canonical unit.
    fn factor(&self) -> Decimal;

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
    const NAME: &'static str = "Unitless";

    fn factor(&self) -> Decimal {
        Decimal::ONE
    }
}

impl<T> PartialEq for Unitless<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Unitless<T> {}

impl<T> fmt::Display for Unitless<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
