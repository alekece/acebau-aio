use std::str::FromStr;

use derive_more::Display;

use super::{Metric, Unit, UnitError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display)]
pub enum LengthUnit {
    #[display("m")]
    Meter,
    #[display("cm")]
    Centimeter,
    #[display("mm")]
    #[default]
    Millimeter,
}

impl Unit for LengthUnit {
    fn factor(&self) -> f32 {
        match self {
            LengthUnit::Meter => 100.,
            LengthUnit::Centimeter => 10.,
            LengthUnit::Millimeter => 1.,
        }
    }
}

pub type Length = Metric<LengthUnit>;

impl Length {
    pub fn from_millimeters(value: f32) -> Self {
        Self::with_unit(value, LengthUnit::Millimeter)
    }

    pub fn from_centimeters(value: f32) -> Self {
        Self::with_unit(value, LengthUnit::Centimeter)
    }

    pub fn from_meters(value: f32) -> Self {
        Self::with_unit(value, LengthUnit::Meter)
    }

    pub fn to_millimeters(self) -> Self {
        self.convert_to(LengthUnit::Millimeter)
    }

    pub fn to_centimeters(self) -> Self {
        self.convert_to(LengthUnit::Centimeter)
    }

    pub fn to_meters(self) -> Self {
        self.convert_to(LengthUnit::Meter)
    }
}

impl FromStr for LengthUnit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(LengthUnit::Meter),
            "cm" => Ok(LengthUnit::Centimeter),
            "mm" => Ok(LengthUnit::Millimeter),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}
