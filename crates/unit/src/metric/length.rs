use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum LengthUnit {
    #[unit(symbol = "m", factor = 1000)]
    Meter,
    #[unit(symbol = "cm", factor = 10)]
    Centimeter,
    #[unit(symbol = "mm", factor = 1)]
    #[default]
    Millimeter,
}
