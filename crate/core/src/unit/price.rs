use std::ops::{Add, AddAssign};

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum PriceError {
    #[snafu(display("Price must be greater than zero"))]
    Negative,
    #[snafu(display("Price must be a finite number"))]
    Infinite,
}

use crate::unit::PercentageDecimal;

fn validate_price(value: f64) -> Result<f64, PriceError> {
    if value.is_infinite() {
        Err(PriceError::Infinite)
    } else if value.is_sign_negative() {
        Err(PriceError::Negative)
    } else {
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct PreTaxPrice(f64);

impl PreTaxPrice {
    fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    pub fn try_new(value: f64) -> Result<Self, PriceError> {
        Ok(Self(validate_price(value)?))
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn include_tax(&self, tax: PercentageDecimal) -> TaxInclusivePrice {
        TaxInclusivePrice::new_unchecked(tax.scale_up(self.0))
    }
}

impl Add for PreTaxPrice {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new_unchecked(self.0 + other.0)
    }
}

impl AddAssign for PreTaxPrice {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct TaxInclusivePrice(f64);

impl TaxInclusivePrice {
    fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    pub fn try_new(value: f64) -> Result<Self, PriceError> {
        Ok(Self(validate_price(value)?))
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn exclude_tax(&self, tax: PercentageDecimal) -> PreTaxPrice {
        PreTaxPrice::new_unchecked(tax.scale_down(self.0))
    }
}

impl Add for TaxInclusivePrice {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::try_new(self.get() + other.get()).unwrap()
    }
}

impl AddAssign for TaxInclusivePrice {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pre_tax_to_tax_inclusive_price() {
        let vat = PercentageDecimal::try_new(20.0).unwrap();
        let price = PreTaxPrice::try_new(100.0).unwrap();

        let price = price.include_tax(vat);
        assert_eq!(120.0, price.get());

        let price = price.exclude_tax(vat);
        assert_eq!(100.0, price.get());
    }
}
