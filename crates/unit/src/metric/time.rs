use crate::Unit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum TimeUnit {
    #[unit(symbol = "y", factor = 8760.)]
    Year,
    #[unit(symbol = "d", factor = 24.)]
    Day,
    #[unit(symbol = "min", factor = 0.016_666_666_666_666_666)]
    Minute,
    #[unit(symbol = "h", factor = 1.)]
    #[default]
    Hour,
}
