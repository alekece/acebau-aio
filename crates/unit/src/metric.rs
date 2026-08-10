pub mod energy;
pub mod length;
pub mod mass;
pub mod percentage;
pub mod power;
pub mod price;
pub mod time;

use std::{
    num::ParseFloatError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use derive_more::Display;
use snafu::{ResultExt, Snafu};

use crate::{
    ratio::Ratio,
    unit::{Unit, Unitless},
};

pub use crate::metric::{
    energy::{Energy, EnergyUnit},
    length::{Length, LengthUnit},
    mass::{Mass, MassUnit},
    percentage::{Percentage, PercentageError, PercentageUnit},
    power::{Power, PowerUnit},
    price::{Price, PriceUnit},
    time::{Time, TimeUnit},
};

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum MetricError {
    #[snafu(display("unexpected '{unit}' unit for unitless metric"))]
    UnexpectedUnit { unit: String },
    #[snafu(display("unknown unit '{unit}'"))]
    UnknownUnit { unit: String },
    #[snafu(display("invalid number '{input}': '{source}'"))]
    NotANumber { input: String, source: ParseFloatError },
    #[snafu(display("invalid format, expected '{format}', got '{input}'"))]
    InvalidFormat { input: String, format: &'static str },
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

    pub fn value(&self) -> f32 {
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
        Ratio::new(self, other)
    }
}

impl<T: Unit> PartialEq for Metric<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.convert_to(self.unit).value
    }
}

impl<T: Unit> Eq for Metric<T> {}

fn split_metric(s: &str) -> (&str, &str) {
    let index = s
        .chars()
        .enumerate()
        .find(|(_, c)| !(c.is_numeric() || *c == '.' || *c == ' '))
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    (s[..index].trim(), s[index..].trim())
}

impl<T> FromStr for Metric<T>
where
    T: Unit + FromStr,
{
    type Err = MetricError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, unit) = split_metric(s);

        if value.is_empty() || unit.is_empty() {
            return Err(MetricError::InvalidFormat {
                input: s.to_string(),
                format: "<value><unit>",
            });
        }

        let value = value.parse::<f32>().context(NotANumberSnafu {
            input: value.to_string(),
        })?;

        let unit = T::from_str(unit).map_err(|_| MetricError::UnknownUnit { unit: unit.to_string() })?;

        Ok(Self::with_unit(value, unit))
    }
}

impl<T> FromStr for Metric<Unitless<T>> {
    type Err = MetricError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, unit) = split_metric(s);

        if value.is_empty() {
            return Err(MetricError::InvalidFormat {
                input: s.to_string(),
                format: "<value>",
            });
        }

        if !unit.is_empty() {
            return Err(MetricError::UnexpectedUnit { unit: unit.to_string() });
        }

        let value = value.parse::<f32>().context(NotANumberSnafu {
            input: value.to_string(),
        })?;

        Ok(Self::new(value))
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
            let value: ::sqlx::types::BigDecimal = self
                .canonicalize()
                .value
                .to_string()
                .parse()
                .map_err(|error| -> BoxDynError { Box::new(error) })?;
            <::sqlx::types::BigDecimal as Encode<'_, Postgres>>::encode_by_ref(&value, buf)
        }
    }

    impl<T: Unit> Decode<'_, Postgres> for Metric<T> {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            let value = <::sqlx::types::BigDecimal as Decode<'_, Postgres>>::decode(value)?
                .to_string()
                .parse::<f32>()?;
            Ok(Self::with_unit(value, T::default()))
        }
    }
}

#[cfg(feature = "graphql")]
mod graphql {
    use std::{borrow::Cow, str::FromStr};

    use async_graphql::{
        ContextSelectionSet, InputType, InputValueError, InputValueResult, OutputType, Positioned, ServerResult, Value,
        parser::types::Field, registry::Registry,
    };

    use super::*;

    impl<T> InputType for Metric<T>
    where
        T: Unit + Send + Sync,
        Metric<T>: FromStr<Err = MetricError>,
    {
        type RawValueType = Self;

        fn type_name() -> Cow<'static, str> {
            "String".into()
        }

        fn create_type_info(registry: &mut Registry) -> String {
            <String as InputType>::create_type_info(registry)
        }

        fn parse(value: Option<Value>) -> InputValueResult<Self> {
            let value = String::parse(value).map_err(|_| InputValueError::custom("expected a string"))?;
            value.parse().map_err(InputValueError::custom)
        }

        fn to_value(&self) -> Value {
            self.to_string().into()
        }

        fn as_raw_value(&self) -> Option<&Self::RawValueType> {
            Some(self)
        }
    }

    impl<T> OutputType for Metric<T>
    where
        T: Unit + Send + Sync,
    {
        fn type_name() -> Cow<'static, str> {
            "String".into()
        }

        fn create_type_info(registry: &mut Registry) -> String {
            <String as OutputType>::create_type_info(registry)
        }

        async fn resolve(&self, _: &ContextSelectionSet<'_>, _: &Positioned<Field>) -> ServerResult<Value> {
            Ok(Value::String(self.to_string()))
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

    impl<'de, T: Unit + FromStr> Deserialize<'de> for Metric<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Self::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
        }
    }

    impl<'de, T> Deserialize<'de> for Metric<Unitless<T>> {
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
    use assert_matches::assert_matches;

    use crate::metric::{Length, Mass};

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
        ] {
            assert_eq!(s.parse::<Mass>(), expected_result);
        }
    }

    #[test]
    fn test_metric_from_str_not_a_number() {
        assert_matches!(" 4..5  g".parse::<Mass>(), Err(MetricError::NotANumber { .. }));
    }

    #[test]
    fn test_metric_from_str_unknown_unit() {
        assert_matches!("20duck".parse::<Mass>(), Err(MetricError::UnknownUnit { .. }));
        assert_matches!("5kgg".parse::<Mass>(), Err(MetricError::UnknownUnit { .. }));
    }

    #[test]
    fn test_metric_from_str_invalid_format() {
        assert_matches!("g".parse::<Mass>(), Err(MetricError::InvalidFormat { .. }));
        assert_matches!("       ".parse::<Mass>(), Err(MetricError::InvalidFormat { .. }));
        assert_matches!("".parse::<Mass>(), Err(MetricError::InvalidFormat { .. }));
        assert_matches!("g45".parse::<Mass>(), Err(MetricError::InvalidFormat { .. }));
    }

    #[test]
    fn test_unitless_metric_from_str() {
        for (s, expected_result) in [("1000", Ok(Price::new(1000.))), ("  3.12   ", Ok(Price::new(3.12)))] {
            assert_eq!(s.parse::<Price>(), expected_result);
        }
    }

    #[test]
    fn test_unitless_metric_from_str_unexpected_unit() {
        assert_matches!("20USD".parse::<Price>(), Err(MetricError::UnexpectedUnit { .. }));
    }

    #[test]
    fn test_metric_canonicalize() {
        for (expected_value, metric) in [
            (8760., Time::from_years(1.)),
            (48., Time::from_days(2.)),
            (6., Time::from_hours(6.)),
        ] {
            assert_eq!(expected_value, metric.canonicalize().value());
        }
    }

    #[test]
    fn test_metric_convert_to() {
        for (unit, metric, expected_metric) in [
            (
                LengthUnit::Meter,
                Length::from_millimeters(1000.),
                Length::from_meters(1.),
            ),
            (
                LengthUnit::Centimeter,
                Length::from_millimeters(10.),
                Length::from_centimeters(1.),
            ),
            (
                LengthUnit::Millimeter,
                Length::from_meters(1.),
                Length::from_millimeters(1000.),
            ),
            (
                LengthUnit::Millimeter,
                Length::from_centimeters(1.),
                Length::from_millimeters(10.),
            ),
            (
                LengthUnit::Centimeter,
                Length::from_meters(1.),
                Length::from_centimeters(100.),
            ),
        ] {
            assert_eq!(expected_metric, metric.convert_to(unit));
        }
    }
}
