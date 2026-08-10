use strum::EnumString;

use super::Metric;
use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Unit)]
pub enum MassUnit {
    #[strum(serialize = "kg")]
    #[unit(symbol = "kg", factor = 1000.)]
    Kilogram,
    #[strum(serialize = "g")]
    #[unit(symbol = "g", factor = 1.)]
    #[default]
    Gram,
}

pub type Mass = Metric<MassUnit>;
