pub mod energy;
pub mod length;
pub mod mass;
pub mod price;
pub mod time;

use std::{
    fmt, ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub, SubAssign}, str::FromStr
};

use derive_more::{Deref, DerefMut, Display};
use snafu::Snafu;
use sqlx::{encode::IsNull, error::BoxDynError, postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

pub trait Unit: Copy + Default + FromStr + fmt::Display {
    fn factor(&self) -> f32;
}

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum UnitError {
    #[snafu(display("Unknown unit '{unit}'"))]
    Unknown { unit: String },
}

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
#[display("{value}{unit}")]
pub struct Metric<T: Unit> {
    value: f32,
    unit: T,
}

impl<T: Unit> Metric<T> {
    pub fn new(value: f32, unit: T) -> Self {
        Self { value, unit }
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn convert_to(&self, unit: T) -> Self {
        let value = self.unit.factor() / unit.factor() * self.value;

        Self { value, unit }
    }

    pub fn unit(&self) -> T {
        self.unit
    }
}

impl<T: Unit> Add for Metric<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        let unit = self.unit();

        self.add_assign(other.convert_to(unit));

        self
    }
}

impl<T: Unit> AddAssign for Metric<T> {
    fn add_assign(&mut self, other: Self) {
        let unit = self.unit();

        self.value.add_assign(other.convert_to(unit).value);
    }
}

impl<T: Unit> Sub for Metric<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        let unit = self.unit();

        self.sub_assign(other.convert_to(unit));

        self
    }
}

impl<T: Unit> SubAssign for Metric<T> {
    fn sub_assign(&mut self, other: Self) {
        let unit = self.unit();

        self.value.sub_assign(other.convert_to(unit).value);
    }
}

impl<T: Unit> Mul<f32> for Metric<T> {
    type Output = Self;

    fn mul(self, value: f32) -> Self::Output {
        Self::new(self.value * value, self.unit)
    }
}

impl<T: Unit> PartialEq for Metric<T> {
    fn eq(&self, other: &Self) -> bool {
        self.convert_to(T::default()).value == other.convert_to(T::default()).value
    }
}

impl<T: Unit> From<f32> for Metric<T> {
    fn from(value: f32) -> Self {
        Self::new(value, T::default())
    }
}

impl<T: Unit> From<Metric<T>> for f32 {
    fn from(metric: Metric<T>) -> f32 {
        metric.convert_to(T::default()).value
    }
}

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
        Ok(Self::from(<f32 as Decode<'_, Postgres>>::decode(value)?))
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

        let value = &s[..index].trim();
        let unit = &s[index..].trim();

        if value.is_empty() || unit.is_empty() {
            return Err(MetricError::InvalidFormat { input: s.to_string() });
        }

        let value = value.parse::<f32>().map_err(|_| MetricError::NotANumber {
            value: value.to_string(),
        })?;

        let unit = T::from_str(unit).map_err(|e| MetricError::UnitError { source: e })?;

        Ok(Self::new(value, unit))
    }
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    impl<T: Unit> Serialize for Metric<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            <String as Serialize>::serialize(&format!("{}", self), serializer)
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
    use crate::types::Mass;

    use super::{MetricError, UnitError};

    #[test]
    fn test_from_str() {
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
}
