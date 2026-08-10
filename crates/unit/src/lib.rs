#![allow(clippy::pedantic)]

extern crate self as acebau_unit;

pub mod metric;
pub mod ratio;
pub mod unit;

pub use acebau_unit_derive::Unit;
pub use metric::{Energy, Length, Mass, Metric, MetricError, Percentage, PercentageError, Power, Price, Time};
pub use ratio::Ratio;
use unit::Unitless;

pub type PricePerTime = Ratio<Unitless<metric::PriceUnit>, metric::TimeUnit>;
pub type PricePerEnergy = Ratio<Unitless<metric::PriceUnit>, metric::EnergyUnit>;
pub type PricePerPower = Ratio<Unitless<metric::PriceUnit>, metric::PowerUnit>;
pub type EnergyPerTime = Ratio<metric::EnergyUnit, metric::TimeUnit>;
pub type PowerPerTime = Ratio<metric::PowerUnit, metric::TimeUnit>;
