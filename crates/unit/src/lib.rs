#![allow(clippy::pedantic)]

pub mod metric;
pub mod ratio;
pub mod unit;

pub use metric::{Energy, Length, Mass, Metric, MetricError, Price, Time};
pub use ratio::Ratio;

pub type PricePerTime = Ratio<Price, Time>;
pub type PricePerEnergy = Ratio<Price, Energy>;
pub type EnergyPerTime = Ratio<Energy, Time>;
