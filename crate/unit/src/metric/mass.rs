use strum::{Display, EnumString};

use super::{Metric, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum Mass {
    #[strum(serialize = "kg")]
    Kilogram,
    #[strum(serialize = "g")]
    #[default]
    Gram,
}

impl Unit for Mass {
    fn factor(&self) -> f32 {
        match self {
            Self::Kilogram => 1000.,
            Self::Gram => 1.,
        }
    }
}

impl Metric<Mass> {
    pub fn from_kilograms(value: f32) -> Self {
        Self::with_unit(value, Mass::Kilogram)
    }

    pub fn from_grams(value: f32) -> Self {
        Self::with_unit(value, Mass::Gram)
    }

    pub fn to_kilograms(self) -> Self {
        self.convert_to(Mass::Kilogram)
    }

    pub fn to_grams(self) -> Self {
        self.convert_to(Mass::Gram)
    }
}
