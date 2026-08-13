use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum MassUnit {
    #[unit(symbol = "kg", factor = 1000)]
    Kilogram,
    #[unit(symbol = "g", factor = 1)]
    #[default]
    Gram,
}
