mod duration;
mod metric;
mod percentage;
mod price;
mod rate;
mod status;

pub use duration::{Duration, DurationError};
pub use metric::{
    energy::{Energy, EnergyUnit},
    length::{Length, LengthUnit},
    mass::{Mass, MassUnit},
    time::{Time, TimeUnit},
    price::{Price, PriceUnit},
    Metric, MetricError, Unit, UnitError,
};
pub use percentage::{Percentage, PercentageError};
pub use price::{PreTaxPrice, PriceError, TaxInclusivePrice};
pub use rate::Rate;
pub use status::Status;

pub type EnergyPerTime = Rate<EnergyUnit, TimeUnit>;
pub type PricePerTime = Rate<PriceUnit, TimeUnit>;
