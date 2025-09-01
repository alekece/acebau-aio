use super::{Metric, Unitless};

#[derive(Debug)]
pub struct PriceTag;

pub type PriceUnit = Unitless<PriceTag>;
pub type Price = Metric<Unitless<PriceUnit>>;
