use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use snafu::Snafu;
use sqlx::Type;

use super::Percentage;

#[derive(Debug, Snafu)]
pub enum PriceError {
    #[snafu(display("Price must be greater than zero"))]
    Negative,
    #[snafu(display("Price must be a finite number"))]
    Infinite,
}

fn validate_price(value: f32) -> Result<f32, PriceError> {
    if value.is_infinite() {
        Err(PriceError::Infinite)
    } else if value.is_sign_negative() {
        Err(PriceError::Negative)
    } else {
        Ok(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Type)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PreTaxPrice(f32);

impl PreTaxPrice {
    fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    pub fn try_new(value: f32) -> Result<Self, PriceError> {
        Ok(Self(validate_price(value)?))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn include_tax(&self, tax: Percentage) -> TaxInclusivePrice {
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

impl Mul<f32> for PreTaxPrice {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new_unchecked(self.0 * other)
    }
}

impl MulAssign<f32> for PreTaxPrice {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Div<f32> for PreTaxPrice {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self::new_unchecked(self.0 / other)
    }
}

impl DivAssign<f32> for PreTaxPrice {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Type)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxInclusivePrice(f32);

impl TaxInclusivePrice {
    fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    pub fn try_new(value: f32) -> Result<Self, PriceError> {
        Ok(Self(validate_price(value)?))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn exclude_tax(&self, tax: Percentage) -> PreTaxPrice {
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
        let vat = Percentage::try_new(20.0).unwrap();
        let price = PreTaxPrice::try_new(100.0).unwrap();

        let price = price.include_tax(vat);
        assert_eq!(120, price.get() as usize);

        let price = price.exclude_tax(vat);
        assert_eq!(100, price.get() as usize);
    }
}
