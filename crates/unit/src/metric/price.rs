use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum PriceUnit {
    #[unit(symbol = "€", factor = 1.)]
    #[default]
    Euro,
}
