use snafu::{Snafu, ensure};

use crate::{Decimal, Unit};

#[derive(Debug, Snafu, PartialEq)]
pub enum PercentageError {
    #[snafu(display("Percentage must be a number between 0 and 100: got {value}"))]
    OutOfBounds { value: Decimal },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum PercentageUnit {
    #[unit(symbol = "%", factor = 1)]
    #[default]
    Percent,
}

impl Percentage {
    pub fn try_new(value: Decimal) -> Result<Self, PercentageError> {
        ensure!(
            (Decimal::ZERO..=Decimal::from(100)).contains(&value),
            OutOfBoundsSnafu { value }
        );

        Ok(Self::with_unit(value, PercentageUnit::Percent))
    }

    pub fn normalize(&self) -> Decimal {
        self.value() / Decimal::from(100)
    }

    pub fn apply_to(&self, value: Decimal) -> Decimal {
        value * self.normalize()
    }

    pub fn scale_down(&self, value: Decimal) -> Decimal {
        value / (Decimal::ONE + self.normalize())
    }

    pub fn scale_up(&self, value: Decimal) -> Decimal {
        value * (Decimal::ONE + self.normalize())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn applies_percentage() {
        let percentage = Percentage::try_new(dec!(20)).unwrap();

        assert_eq!(dec!(0.2), percentage.normalize());
        assert_eq!(dec!(20), percentage.apply_to(dec!(100)));
        assert_eq!(dec!(100), percentage.scale_down(dec!(120)));
        assert_eq!(dec!(120), percentage.scale_up(dec!(100)));
    }

    #[test]
    fn rejects_out_of_bounds_percentage() {
        assert!(Percentage::try_new(dec!(-1)).is_err());
        assert!(Percentage::try_new(dec!(101)).is_err());
    }
}
