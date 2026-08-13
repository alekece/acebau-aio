use snafu::{Snafu, ensure};

use crate::Unit;

#[derive(Debug, Snafu, PartialEq)]
pub enum PercentageError {
    #[snafu(display("Percentage must be a number between 0 and 100: got {value}"))]
    OutOfBounds { value: f32 },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Unit)]
pub enum PercentageUnit {
    #[unit(symbol = "%", factor = 1.)]
    #[default]
    Percent,
}

impl Percentage {
    pub fn try_new(value: f32) -> Result<Self, PercentageError> {
        ensure!((0.0..=100.0).contains(&value), OutOfBoundsSnafu { value });

        Ok(Self::with_unit(value, PercentageUnit::Percent))
    }

    pub fn normalize(&self) -> f32 {
        self.value() / 100.0
    }

    pub fn apply_to(&self, value: f32) -> f32 {
        value * self.normalize()
    }

    pub fn scale_down(&self, value: f32) -> f32 {
        value / (1.0 + self.normalize())
    }

    pub fn scale_up(&self, value: f32) -> f32 {
        value * (1.0 + self.normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_percentage() {
        let percentage = Percentage::try_new(20.0).unwrap();

        assert_eq!(0.2, percentage.normalize());
        assert!((percentage.apply_to(100.0) - 20.0).abs() < f32::EPSILON);
        assert!((percentage.scale_down(120.0) - 100.0).abs() < 0.001);
        assert!((percentage.scale_up(100.0) - 120.0).abs() < 0.001);
    }

    #[test]
    fn rejects_out_of_bounds_percentage() {
        assert!(Percentage::try_new(-1.0).is_err());
        assert!(Percentage::try_new(101.0).is_err());
    }
}
