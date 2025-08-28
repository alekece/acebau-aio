use std::str::FromStr;

use derive_more::Display;

use super::{Metric, Unit, UnitError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display)]
pub enum PriceUnit {
    #[display("€")]
    #[default]
    Euro,
}

impl Unit for PriceUnit {
    fn factor(&self) -> f32 {
        match self {
            PriceUnit::Euro => 1.,
        }
    }
}

pub type Price = Metric<PriceUnit>;

impl Price {
    pub fn from_euros(value: f32) -> Self {
        Self::new(value, PriceUnit::Euro)
    }

    pub fn to_euros(self) -> Self {
        self.convert_to(PriceUnit::Euro)
    }
}

impl FromStr for PriceUnit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "€" => Ok(PriceUnit::Euro),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}
