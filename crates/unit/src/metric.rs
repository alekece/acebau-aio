pub mod length;
pub mod mass;
pub mod percentage;
pub mod power;
pub mod price;
pub mod time;

use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use derive_more::Display;
use rust_decimal::{Decimal, Error as DecimalError};
use snafu::{ResultExt, Snafu};

use crate::{
    ratio::Ratio,
    unit::{Unit, Unitless},
};

pub use crate::metric::{
    length::{Length, LengthUnit},
    mass::{Mass, MassUnit},
    percentage::{Percentage, PercentageError, PercentageUnit},
    power::{Power, PowerUnit},
    price::{Price, PriceUnit},
    time::{Time, TimeUnit},
};

#[derive(Debug, Snafu, PartialEq)]
pub enum MetricError {
    #[snafu(display("unexpected '{unit}' unit for unitless metric"))]
    UnexpectedUnit { unit: String },
    #[snafu(display("unknown unit '{unit}'"))]
    UnknownUnit { unit: String },
    #[snafu(display("invalid number '{input}': '{source}'"))]
    NotANumber { input: String, source: DecimalError },
    #[snafu(display("cannot represent floating-point value '{value}' as a decimal"))]
    InvalidFloat { value: f64 },
    #[snafu(display("invalid format, expected '{format}', got '{input}'"))]
    InvalidFormat { input: String, format: &'static str },
}

#[derive(Debug, Copy, Clone, Display)]
#[display("{}{}", value, unit.to_string())]
pub struct Metric<T: Unit> {
    value: Decimal,
    unit: T,
}

impl<T: Unit> Metric<T> {
    pub fn with_unit<V: Into<Decimal>>(value: V, unit: T) -> Self {
        Self {
            value: value.into(),
            unit,
        }
    }

    pub fn try_with_unit(value: f64, unit: T) -> Result<Self, MetricError> {
        let value = Decimal::try_from(value).map_err(|_| MetricError::InvalidFloat { value })?;
        Ok(Self::with_unit(value, unit))
    }

    pub fn value(&self) -> Decimal {
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
    pub fn new<V: Into<Decimal>>(value: V) -> Self {
        Self::with_unit(value, Unitless::default())
    }

    pub fn try_new(value: f64) -> Result<Self, MetricError> {
        Self::try_with_unit(value, Unitless::default())
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

impl<T: Unit> Mul<Decimal> for Metric<T> {
    type Output = Self;

    fn mul(mut self, value: Decimal) -> Self::Output {
        self.mul_assign(value);

        self
    }
}

impl<T: Unit> MulAssign<Decimal> for Metric<T> {
    fn mul_assign(&mut self, value: Decimal) {
        self.value.mul_assign(value);
    }
}

impl<T: Unit> Div<Decimal> for Metric<T> {
    type Output = Self;

    fn div(mut self, value: Decimal) -> Self::Output {
        self.div_assign(value);

        self
    }
}

impl<T: Unit> DivAssign<Decimal> for Metric<T> {
    fn div_assign(&mut self, value: Decimal) {
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

        let value = value.parse::<Decimal>().context(NotANumberSnafu {
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

        let value = value.parse::<Decimal>().context(NotANumberSnafu {
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
                .parse::<Decimal>()?;
            Ok(Self::with_unit(value, T::default()))
        }
    }
}

#[cfg(feature = "graphql")]
mod graphql {
    use std::{borrow::Cow, str::FromStr};

    use async_graphql::{
        ContextSelectionSet, InputObject, InputType, InputValueError, InputValueResult, OutputType, Positioned,
        ServerResult, Value,
        indexmap::IndexMap,
        parser::types::Field,
        registry::{Deprecation, MetaInputValue, MetaType, MetaTypeId, Registry},
    };

    use super::*;

    #[derive(InputObject)]
    struct MetricInput {
        value: String,
        unit: String,
    }

    impl<T> InputType for Metric<T>
    where
        T: Unit + Send + Sync,
        Metric<T>: FromStr<Err = MetricError>,
    {
        type RawValueType = Self;

        fn type_name() -> Cow<'static, str> {
            format!("{}MetricInput", T::NAME).into()
        }

        fn create_type_info(registry: &mut Registry) -> String {
            registry.create_input_type::<Self, _>(MetaTypeId::InputObject, |registry| MetaType::InputObject {
                name: <Self as InputType>::type_name().into_owned(),
                description: None,
                input_fields: ["value", "unit"]
                    .into_iter()
                    .map(|name| {
                        (
                            name.to_owned(),
                            MetaInputValue {
                                name: name.to_owned(),
                                description: None,
                                ty: <String as InputType>::create_type_info(registry),
                                deprecation: Deprecation::NoDeprecated,
                                default_value: None,
                                visible: None,
                                inaccessible: false,
                                tags: Vec::new(),
                                is_secret: false,
                                directive_invocations: Vec::new(),
                            },
                        )
                    })
                    .collect::<IndexMap<_, _>>(),
                visible: None,
                inaccessible: false,
                tags: Vec::new(),
                rust_typename: Some(std::any::type_name::<Self>()),
                oneof: false,
                directive_invocations: Vec::new(),
            })
        }

        fn parse(value: Option<Value>) -> InputValueResult<Self> {
            let input = MetricInput::parse(value).map_err(InputValueError::propagate)?;
            format!("{}{}", input.value, input.unit)
                .parse()
                .map_err(InputValueError::custom)
        }

        fn to_value(&self) -> Value {
            MetricInput {
                value: self.value.to_string(),
                unit: self.unit.to_string(),
            }
            .to_value()
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

    #[cfg(test)]
    mod tests {
        use crate::metric::{Power, Price, Time};

        use super::*;

        #[test]
        fn graphql_input_names_include_the_unit_name() {
            assert_eq!(<Price as InputType>::type_name(), "PriceMetricInput");
            assert_eq!(<Time as InputType>::type_name(), "TimeMetricInput");
            assert_eq!(<Power as InputType>::type_name(), "PowerMetricInput");
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
    use rust_decimal::dec;

    use crate::metric::{Length, Mass};

    use super::*;

    #[test]
    fn test_metric_from_str() {
        for (s, expected_result) in [
            ("1000g", Ok(Mass::from_grams(dec!(1000)))),
            ("2kg", Ok(Mass::from_kilograms(dec!(2)))),
            ("1.5 kg", Ok(Mass::from_kilograms(dec!(1.5)))),
            ("   4kg", Ok(Mass::from_kilograms(dec!(4)))),
            ("2kg     ", Ok(Mass::from_kilograms(dec!(2)))),
            ("     45      g       ", Ok(Mass::from_grams(dec!(45)))),
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
    fn test_price_from_str() {
        for (s, expected_result) in [
            ("1000€", Ok(Price::from_euros(dec!(1000)))),
            ("  3.12 €  ", Ok(Price::from_euros(dec!(3.12)))),
        ] {
            assert_eq!(s.parse::<Price>(), expected_result);
        }
    }

    #[test]
    fn test_price_from_str_unknown_unit() {
        assert_matches!("20USD".parse::<Price>(), Err(MetricError::UnknownUnit { .. }));
    }

    #[test]
    fn test_generated_float_constructor() {
        assert_eq!(Time::from_hours(dec!(2.5)), Time::try_from_hours(2.5).unwrap());
        assert_matches!(Time::try_from_hours(f64::NAN), Err(MetricError::InvalidFloat { .. }));
        assert_matches!(
            Time::try_from_hours(f64::INFINITY),
            Err(MetricError::InvalidFloat { .. })
        );
    }

    #[test]
    fn test_metric_canonicalize() {
        for (expected_value, metric) in [
            (dec!(525600), Time::from_years(dec!(1))),
            (dec!(2880), Time::from_days(dec!(2))),
            (dec!(360), Time::from_hours(dec!(6))),
        ] {
            assert_eq!(expected_value, metric.canonicalize().value());
        }
    }

    #[test]
    fn test_metric_convert_to() {
        for (unit, metric, expected_metric) in [
            (
                LengthUnit::Meter,
                Length::from_millimeters(dec!(1000)),
                Length::from_meters(dec!(1)),
            ),
            (
                LengthUnit::Centimeter,
                Length::from_millimeters(dec!(10)),
                Length::from_centimeters(dec!(1)),
            ),
            (
                LengthUnit::Millimeter,
                Length::from_meters(dec!(1)),
                Length::from_millimeters(dec!(1000)),
            ),
            (
                LengthUnit::Millimeter,
                Length::from_centimeters(dec!(1)),
                Length::from_millimeters(dec!(10)),
            ),
            (
                LengthUnit::Centimeter,
                Length::from_meters(dec!(1)),
                Length::from_centimeters(dec!(100)),
            ),
        ] {
            assert_eq!(expected_metric, metric.convert_to(unit));
        }
    }
}
