use num::{Float, Num, NumCast, PrimInt};
use snafu::Snafu;

#[derive(Debug, PartialEq, Eq, Snafu)]
pub enum PercentageError {
    #[snafu(display("Percentage must be between 0 and 100"))]
    OutOfBounds,
}

fn validate_percentage<T: Num + NumCast + Copy>(value: T) -> Result<T, PercentageError> {
    let floating_value: f32 = NumCast::from(value).unwrap();

    (0.0..=100.0)
        .contains(&floating_value)
        .then_some(value)
        .ok_or(PercentageError::OutOfBounds)
}

fn apply_to<T: Num + NumCast>(value: T, percentage: T) -> T {
    value * percentage / NumCast::from(100).unwrap()
}

fn scale_down<T: Num + NumCast>(value: T, percentage: T) -> T {
    let percentage: f64 = NumCast::from(percentage).unwrap();
    let value: f64 = NumCast::from(value).unwrap();

    T::from(value / (1.0 + percentage / 100.0)).unwrap()
}

fn scale_up<T: Num + NumCast + Copy>(value: T, percentage: T) -> T {
    value + apply_to(value, percentage)
}

/// `PercentageDecimal`, a type representing a percentage value for any floating point type.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct PercentageDecimal(f32);

impl PercentageDecimal {
    pub fn try_new(value: f32) -> Result<Self, PercentageError> {
        Ok(Self(validate_percentage(value)?))
    }

    fn as_float<T: Float>(&self) -> T {
        T::from(self.0).unwrap()
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    /// Apply the percentage to a value.
    pub fn apply_to<T: Float>(&self, value: T) -> T {
        apply_to(value, self.as_float())
    }

    /// Scale down a value by the percentage.
    pub fn scale_down<T: Float>(&self, value: T) -> T {
        scale_down(value, self.as_float())
    }

    /// Scale up a value by the percentage.
    pub fn scale_up<T: Float>(&self, value: T) -> T {
        scale_up(value, self.as_float())
    }
}

/// `PercentageInteger`, a type representing a percentage value for any integer type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct PercentageInteger(u8);

impl PercentageInteger {
    pub fn try_new(value: u8) -> Result<Self, PercentageError> {
        Ok(Self(validate_percentage(value)?))
    }

    fn as_integer<T: PrimInt>(&self) -> T {
        T::from(self.0).unwrap()
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    /// Apply the percentage to a value.
    pub fn apply_to<T: PrimInt>(&self, value: T) -> T {
        apply_to(value, self.as_integer())
    }

    /// Scale down a value by the percentage.
    pub fn scale_down<T: PrimInt>(&self, value: T) -> T {
        scale_down(value, self.as_integer())
    }

    /// Scale up a value by the percentage.
    pub fn scale_up<T: PrimInt>(&self, value: T) -> T {
        scale_up(value, self.as_integer())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage_apply_to() {
        let percentage = PercentageDecimal::try_new(50.0).unwrap();
        assert_eq!(percentage.apply_to(100.0f32), 50.0);
        assert_eq!(percentage.apply_to(200.0f64), 100.0);

        let percentage = PercentageInteger::try_new(50).unwrap();
        assert_eq!(percentage.apply_to(100u16), 50);
        assert_eq!(percentage.apply_to(200u32), 100);
        assert_eq!(percentage.apply_to(400u64), 200);
        assert_eq!(percentage.apply_to(800usize), 400);
    }

    #[test]
    fn percentage_scale_up() {
        let percentage = PercentageDecimal::try_new(20.5).unwrap();
        assert_eq!(percentage.scale_up(100.0), 120.5);
        assert_eq!(percentage.scale_up(200.0), 241.0);

        let percentage = PercentageInteger::try_new(15).unwrap();
        assert_eq!(percentage.scale_up(100), 115);
        assert_eq!(percentage.scale_up(200), 230);
    }

    #[test]
    fn percentage_scale_down() {
        let percentage = PercentageDecimal::try_new(20.0).unwrap();
        assert_eq!(percentage.scale_down(120.0), 100.0);
        assert_eq!(percentage.scale_down(240.0), 200.0);

        let percentage = PercentageInteger::try_new(15).unwrap();
        assert_eq!(percentage.scale_down(115), 100);
        assert_eq!(percentage.scale_down(230), 200);
    }

    #[test]
    fn percentage_out_of_bounds() {
        assert_eq!(Err(PercentageError::OutOfBounds), PercentageDecimal::try_new(-10.0));
        assert_eq!(Err(PercentageError::OutOfBounds), PercentageDecimal::try_new(1000.0));
        assert_eq!(Err(PercentageError::OutOfBounds), PercentageInteger::try_new(150));
    }
}
