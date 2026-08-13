use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum PowerUnit {
    #[unit(symbol = "kW", factor = 1000.)]
    Kilowatt,
    #[unit(symbol = "W", factor = 1.)]
    #[default]
    Watt,
}
