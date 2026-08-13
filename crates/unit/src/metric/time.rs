use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum TimeUnit {
    #[unit(symbol = "y", factor = 525_600.)]
    Year,
    #[unit(symbol = "d", factor = 1440.)]
    Day,
    #[unit(symbol = "h", factor = 60.)]
    Hour,
    #[unit(symbol = "min", factor = 1.)]
    #[default]
    Minute,
}
