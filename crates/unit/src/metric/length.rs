use strum::EnumString;

use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Unit)]
pub enum LengthUnit {
    #[strum(serialize = "m")]
    #[unit(symbol = "m", factor = 1000.)]
    Meter,
    #[strum(serialize = "cm")]
    #[unit(symbol = "cm", factor = 10.)]
    Centimeter,
    #[strum(serialize = "mm")]
    #[unit(symbol = "mm", factor = 1.)]
    #[default]
    Millimeter,
}
