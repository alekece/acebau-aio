pub mod energy;
pub mod length;
pub mod mass;
pub mod price;
pub mod time;

use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use derive_more::Display;
use snafu::Snafu;

use crate::{
    ratio::Ratio,
    unit::{Unit, UnitError, Unitless},
};

pub use crate::metric::{
    energy::{Energy, EnergyUnit},
    length::{Length, LengthUnit},
    mass::{Mass, MassUnit},
    price::{Price, PriceUnit},
    time::{Time, TimeUnit},
};

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum MetricError {
    #[snafu(display("Unit error: '{source}'"))]
    UnitError { source: UnitError },
    #[snafu(display("Value must be a floating point number, got '{value}'"))]
    NotANumber { value: String },
    #[snafu(display("Invalid format, expected '<value><unit>', got '{input}'"))]
    InvalidFormat { input: String },
}

#[derive(Debug, Copy, Clone, Display)]
#[display("{}{}", value, unit.to_string())]
pub struct Metric<T: Unit> {
    value: f32,
    unit: T,
}

impl<T: Unit> Metric<T> {
    pub fn with_unit(value: f32, unit: T) -> Self {
        Self { value, unit }
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn canonicalize(self) -> Self {
        self.convert_to(T::default())
    }

    pub fn convert_to(self, unit: T) -> Self {
        Self {
            value: self.value * self.unit.factor() / unit.factor(),
            unit,
        }
    }

    pub fn unit(&self) -> T {
        self.unit
    }
}

impl<T> Metric<Unitless<T>> {
    pub fn new(value: f32) -> Self {
        Self::with_unit(value, Unitless::default())
    }
}

impl<T: Unit> Add for Metric<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.add_assign(other);

        self
    }
}

impl<T: Unit> AddAssign for Metric<T> {
    fn add_assign(&mut self, other: Self) {
        self.value.add_assign(other.convert_to(self.unit).value);
    }
}
impl<T: Unit> Sub for Metric<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.sub_assign(other);

        self
    }
}

impl<T: Unit> SubAssign for Metric<T> {
    fn sub_assign(&mut self, other: Self) {
        self.value.sub_assign(other.convert_to(self.unit).value);
    }
}

impl<T: Unit> Mul<f32> for Metric<T> {
    type Output = Self;

    fn mul(mut self, value: f32) -> Self::Output {
        self.mul_assign(value);

        self
    }
}

impl<T: Unit> MulAssign<f32> for Metric<T> {
    fn mul_assign(&mut self, value: f32) {
        self.value.mul_assign(value);
    }
}

impl<T: Unit> Div<f32> for Metric<T> {
    type Output = Self;

    fn div(mut self, value: f32) -> Self::Output {
        self.div_assign(value);

        self
    }
}

impl<T: Unit> DivAssign<f32> for Metric<T> {
    fn div_assign(&mut self, value: f32) {
        self.value.div_assign(value);
    }
}

impl<T: Unit, U: Unit> Div<Metric<U>> for Metric<T> {
    type Output = Ratio<T, U>;

    fn div(self, other: Metric<U>) -> Self::Output {
        Ratio::new(self.value / other.value, self.unit, other.unit)
    }
}

impl<T: Unit> PartialEq for Metric<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.convert_to(self.unit).value
    }
}

impl<T> FromStr for Metric<T>
where
    T: Unit + FromStr<Err = UnitError>,
{
    type Err = MetricError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s
            .chars()
            .enumerate()
            .find(|(_, c)| !(c.is_numeric() || *c == '.' || *c == ' '))
            .map(|(i, _)| i)
            .unwrap_or_default();

        let value = s[..index].trim();
        let unit = s[index..].trim();

        if value.is_empty() {
            return Err(MetricError::InvalidFormat { input: s.to_string() });
        }

        let value = value.parse::<f32>().map_err(|_| MetricError::NotANumber {
            value: value.to_string(),
        })?;

        let unit = T::from_str(unit).map_err(|e| MetricError::UnitError { source: e })?;

        Ok(Self::with_unit(value, unit))
    }
}

#[cfg(feature = "sqlx")]
mod sqlx {
    use ::sqlx::{Database, Decode, Encode, Postgres, Type, encode::IsNull, error::BoxDynError, postgres::PgTypeInfo};

    use super::*;

    impl<T: Unit> Type<Postgres> for Metric<T> {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::with_name("numeric")
        }
    }

    impl<T: Unit> Encode<'_, Postgres> for Metric<T> {
        fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
            <f32 as Encode<'_, Postgres>>::encode_by_ref(&self.convert_to(T::default()).value, buf)
        }
    }

    impl<T: Unit> Decode<'_, Postgres> for Metric<T> {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            Ok(Self::with_unit(
                <f32 as Decode<'_, Postgres>>::decode(value)?,
                T::default(),
            ))
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    use super::*;

    impl<T: Unit> Serialize for Metric<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            <String as Serialize>::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, T: Unit + FromStr<Err = UnitError>> Deserialize<'de> for Metric<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Self::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::metric::Mass;

    use super::*;

    #[test]
    fn test_metric_from_str() {
        for (s, expected_result) in [
            ("1000g", Ok(Mass::from_grams(1000.))),
            ("2kg", Ok(Mass::from_kilograms(2.))),
            ("1.5 kg", Ok(Mass::from_kilograms(1.5))),
            ("   4kg", Ok(Mass::from_kilograms(4.))),
            ("2kg     ", Ok(Mass::from_kilograms(2.))),
            ("     45      g       ", Ok(Mass::from_grams(45.))),
            (
                " 4..5  g",
                Err(MetricError::NotANumber {
                    value: "4..5".to_string(),
                }),
            ),
            (
                " 20duck",
                Err(MetricError::UnitError {
                    source: UnitError::Unknown {
                        unit: "duck".to_string(),
                    },
                }),
            ),
            (
                "   ",
                Err(MetricError::InvalidFormat {
                    input: "   ".to_string(),
                }),
            ),
            ("", Err(MetricError::InvalidFormat { input: "".to_string() })),
            (
                "g45",
                Err(MetricError::InvalidFormat {
                    input: "g45".to_string(),
                }),
            ),
        ] {
            assert_eq!(s.parse::<Mass>(), expected_result);
        }
    }

    #[test]
    fn test_metric_canonicalize() {
        for (expected_value, metric) in [
            (8760., Time::from_years(1.)),
            (48., Time::from_days(2.)),
            (6., Time::from_hours(6.)),
        ] {
            assert_eq!(expected_value, metric.canonicalize().get());
        }
    }
}
