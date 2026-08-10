use strum::EnumString;

use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Unit)]
pub enum TimeUnit {
    #[strum(serialize = "y")]
    #[unit(symbol = "y", factor = 8760.)]
    Year,
    #[strum(serialize = "d")]
    #[unit(symbol = "d", factor = 24.)]
    Day,
    #[strum(serialize = "h")]
    #[unit(symbol = "h", factor = 1.)]
    #[default]
    Hour,
}
