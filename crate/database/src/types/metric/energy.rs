use std::str::FromStr;

use derive_more::Display;

use super::{Metric, Unit, UnitError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display)]
pub enum EnergyUnit {
    #[display("kW")]
    Kilowatt,
    #[display("W")]
    #[default]
    Watt,
}

impl Unit for EnergyUnit {
    fn factor(&self) -> f32 {
        match self {
            EnergyUnit::Kilowatt => 1000.,
            EnergyUnit::Watt => 1.,
        }
    }
}

pub type Energy = Metric<EnergyUnit>;

impl Energy {
    pub fn from_kilowatts(value: f32) -> Self {
        Self::new(value, EnergyUnit::Kilowatt)
    }

    pub fn from_watts(value: f32) -> Self {
        Self::new(value, EnergyUnit::Watt)
    }

    pub fn to_kilowatts(self) -> Self {
        self.convert_to(EnergyUnit::Kilowatt)
    }

    pub fn to_watts(self) -> Self {
        self.convert_to(EnergyUnit::Watt)
    }
}

impl FromStr for EnergyUnit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kW" => Ok(EnergyUnit::Kilowatt),
            "W" => Ok(EnergyUnit::Watt),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}
