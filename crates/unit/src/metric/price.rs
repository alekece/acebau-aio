use crate::{metric::Metric, unit::Unitless};

#[derive(Debug)]
pub struct PriceUnit;

pub type Price = Metric<Unitless<PriceUnit>>;
