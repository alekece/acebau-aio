use strum::{Display, EnumString};

use super::{Metric, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum EnergyUnit {
    #[strum(serialize = "kW")]
    Kilowatt,
    #[strum(serialize = "W")]
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
        Self::with_unit(value, EnergyUnit::Kilowatt)
    }

    pub fn from_watts(value: f32) -> Self {
        Self::with_unit(value, EnergyUnit::Watt)
    }

    pub fn to_kilowatts(self) -> Self {
        self.convert_to(EnergyUnit::Kilowatt)
    }

    pub fn to_watts(self) -> Self {
        self.convert_to(EnergyUnit::Watt)
    }
}
