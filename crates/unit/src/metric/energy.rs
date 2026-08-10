use strum::EnumString;

use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Unit)]
pub enum EnergyUnit {
    #[strum(serialize = "kW")]
    #[unit(symbol = "kW", factor = 1000.)]
    Kilowatt,
    #[strum(serialize = "W")]
    #[unit(symbol = "W", factor = 1.)]
    #[default]
    Watt,
}
