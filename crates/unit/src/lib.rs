#![allow(clippy::pedantic)]

extern crate self as acebau_unit;

pub mod metric;
pub mod ratio;
pub mod unit;

pub use acebau_unit_derive::Unit;
pub use metric::{Length, Mass, Metric, MetricError, Percentage, PercentageError, Power, Price, Time};
pub use ratio::Ratio;

use metric::{PowerUnit, PriceUnit, TimeUnit};

pub type PricePerTime = Ratio<PriceUnit, TimeUnit>;
pub type PricePerPower = Ratio<PriceUnit, PowerUnit>;
pub type PowerPerTime = Ratio<PowerUnit, TimeUnit>;
