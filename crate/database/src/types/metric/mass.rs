use std::str::FromStr;

use derive_more::{Display};

use super::{Metric, Unit, UnitError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display)]
pub enum MassUnit {
    #[display("kg")]
    Kilogram,
    #[display("g")]
    #[default]
    Gram,
}

impl Unit for MassUnit {
    fn factor(&self) -> f32 {
        match self {
            MassUnit::Kilogram => 1000.,
            MassUnit::Gram => 1.,
        }
    }
}

pub type Mass = Metric<MassUnit>;

impl Mass {
    pub fn from_kilograms(value: f32) -> Self {
        Self::new(value, MassUnit::Kilogram)
    }

    pub fn from_grams(value: f32) -> Self {
        Self::new(value, MassUnit::Gram)
    }

    pub fn to_kilograms(self) -> Self {
        self.convert_to(MassUnit::Kilogram)
    }

    pub fn to_grams(self) -> Self {
        self.convert_to(MassUnit::Gram)
    }
}

impl FromStr for MassUnit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kg" => Ok(MassUnit::Kilogram),
            "g" => Ok(MassUnit::Gram),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}
