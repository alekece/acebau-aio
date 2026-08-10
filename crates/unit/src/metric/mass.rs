use strum::{Display, EnumString};

use super::{Metric, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum MassUnit {
    #[strum(serialize = "kg")]
    Kilogram,
    #[strum(serialize = "g")]
    #[default]
    Gram,
}

impl Unit for MassUnit {
    fn factor(&self) -> f32 {
        match self {
            Self::Kilogram => 1000.,
            Self::Gram => 1.,
        }
    }
}

pub type Mass = Metric<MassUnit>;

impl Mass {
    pub fn from_kilograms(value: f32) -> Self {
        Self::with_unit(value, MassUnit::Kilogram)
    }

    pub fn from_grams(value: f32) -> Self {
        Self::with_unit(value, MassUnit::Gram)
    }

    pub fn to_kilograms(self) -> Self {
        self.convert_to(MassUnit::Kilogram)
    }

    pub fn to_grams(self) -> Self {
        self.convert_to(MassUnit::Gram)
    }
}
