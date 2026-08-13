use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use derive_more::Display;
use rust_decimal::Decimal;

use crate::{
    metric::{Metric, MetricError},
    unit::Unit,
};

#[derive(Debug, Copy, Clone, Display, PartialEq, Eq)]
#[display("{numerator}/{denominator}")]
pub struct Ratio<T: Unit, U: Unit> {
    numerator: Metric<T>,
    denominator: Metric<U>,
}

impl<T: Unit, U: Unit> Ratio<T, U> {
    pub fn new(numerator: Metric<T>, denominator: Metric<U>) -> Self {
        Self {
            numerator: numerator / denominator.value(),
            denominator: Metric::with_unit(Decimal::ONE, denominator.unit()),
        }
    }

    pub fn with_units<V: Into<Decimal>>(value: V, numerator: T, denominator: U) -> Self {
        Self {
            numerator: Metric::with_unit(value, numerator),
            denominator: Metric::with_unit(Decimal::ONE, denominator),
        }
    }

    pub fn try_with_units(value: f64, numerator: T, denominator: U) -> Result<Self, MetricError> {
        Ok(Self {
            numerator: Metric::try_with_unit(value, numerator)?,
            denominator: Metric::with_unit(Decimal::ONE, denominator),
        })
    }

    pub fn get(&self) -> Decimal {
        self.numerator.value()
    }

    pub fn numerator_unit(&self) -> T {
        self.numerator.unit()
    }

    pub fn denominator_unit(&self) -> U {
        self.denominator.unit()
    }

    pub fn convert_to(self, numerator: T, denominator: U) -> Self {
        Self::with_units(
            self.numerator.value() * self.numerator.unit().factor() / numerator.factor() * denominator.factor()
                / self.denominator.unit().factor(),
            numerator,
            denominator,
        )
    }

    pub fn canonicalize(self) -> Self {
        self.convert_to(T::default(), U::default())
    }
}

impl<T: Unit, U: Unit> AddAssign for Ratio<T, U> {
    fn add_assign(&mut self, other: Self) {
        self.numerator += other
            .convert_to(self.numerator.unit(), self.denominator.unit())
            .numerator;
    }
}

impl<T: Unit, U: Unit> Add for Ratio<T, U> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.add_assign(other);

        self
    }
}

impl<T: Unit, U: Unit> SubAssign for Ratio<T, U> {
    fn sub_assign(&mut self, other: Self) {
        self.numerator -= other
            .convert_to(self.numerator.unit(), self.denominator.unit())
            .numerator;
    }
}

impl<T: Unit, U: Unit> Sub for Ratio<T, U> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self.sub_assign(other);

        self
    }
}

impl<T: Unit, U: Unit> Mul<Decimal> for Ratio<T, U> {
    type Output = Self;

    fn mul(mut self, value: Decimal) -> Self::Output {
        self.numerator *= value;

        self
    }
}

impl<T: Unit, U: Unit> MulAssign<Decimal> for Ratio<T, U> {
    fn mul_assign(&mut self, value: Decimal) {
        self.numerator *= value;
    }
}

impl<T: Unit, U: Unit> Div<Decimal> for Ratio<T, U> {
    type Output = Self;

    fn div(mut self, value: Decimal) -> Self::Output {
        self.numerator /= value;

        self
    }
}

impl<T: Unit, U: Unit> DivAssign<Decimal> for Ratio<T, U> {
    fn div_assign(&mut self, value: Decimal) {
        self.numerator /= value;
    }
}

impl<T, U> Mul<Metric<U>> for Ratio<T, U>
where
    T: Unit,
    U: Unit,
{
    type Output = Metric<T>;

    fn mul(self, metric: Metric<U>) -> Self::Output {
        Metric::with_unit(
            self.numerator.value() * metric.convert_to(self.denominator.unit()).value(),
            self.numerator.unit(),
        )
    }
}

impl<T: Unit, U: Unit, V: Unit> Mul<Ratio<U, V>> for Ratio<T, U> {
    type Output = Ratio<T, V>;

    fn mul(self, other: Ratio<U, V>) -> Self::Output {
        Ratio::new(
            self.numerator * other.numerator.convert_to(self.denominator.unit()).value(),
            other.denominator,
        )
    }
}

impl<T: Unit, U: Unit, V: Unit> Div<Ratio<U, V>> for Ratio<T, V> {
    type Output = Ratio<T, U>;

    fn div(self, other: Ratio<U, V>) -> Self::Output {
        Ratio::new(
            self.numerator
                / (other.numerator.value() * self.denominator.unit().factor() / other.denominator.unit().factor()),
            Metric::with_unit(Decimal::ONE, other.numerator.unit()),
        )
    }
}

impl<T, U> FromStr for Ratio<T, U>
where
    T: Unit,
    U: Unit,
    Metric<T>: FromStr<Err = MetricError>,
    Metric<U>: FromStr<Err = MetricError>,
{
    type Err = MetricError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((numerator, denominator)) = s.split_once("/") else {
            return Err(MetricError::InvalidFormat {
                input: s.to_string(),
                format: "<metric>/<metric>",
            });
        };

        Ok(Self::new(numerator.parse()?, denominator.parse()?))
    }
}

#[cfg(feature = "sqlx")]
mod sqlx {
    use ::sqlx::{Database, Decode, Encode, Postgres, Type, encode::IsNull, error::BoxDynError, postgres::PgTypeInfo};

    use super::*;

    impl<T: Unit, U: Unit> Type<Postgres> for Ratio<T, U> {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::with_name("numeric")
        }
    }

    impl<T, U> Encode<'_, Postgres> for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
    {
        fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
            let value: ::sqlx::types::BigDecimal = self
                .canonicalize()
                .numerator
                .value()
                .to_string()
                .parse()
                .map_err(|error| -> BoxDynError { Box::new(error) })?;
            <::sqlx::types::BigDecimal as Encode<'_, Postgres>>::encode_by_ref(&value, buf)
        }
    }

    impl<T, U> Decode<'_, Postgres> for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
    {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            let value = <::sqlx::types::BigDecimal as Decode<'_, Postgres>>::decode(value)?
                .to_string()
                .parse::<Decimal>()?;
            Ok(Self::new(
                Metric::with_unit(value, T::default()),
                Metric::with_unit(Decimal::ONE, U::default()),
            ))
        }
    }
}

#[cfg(feature = "graphql")]
mod graphql {
    use std::{borrow::Cow, marker::PhantomData, str::FromStr};

    use async_graphql::{
        ContextSelectionSet, InputObject, InputType, InputValueError, InputValueResult, OutputType, Positioned,
        ServerResult, SimpleObject, TypeName, Value, indexmap::IndexMap, parser::types::Field, registry::Registry,
    };

    use super::*;
    use crate::graphql;

    #[derive(InputObject)]
    struct RatioInput {
        value: Decimal,
        numerator_unit: String,
        denominator_unit: String,
    }

    #[derive(SimpleObject)]
    #[graphql(name_type)]
    struct RatioOutput<T: Unit + Send + Sync, U: Unit + Send + Sync> {
        value: Decimal,
        numerator_unit: String,
        denominator_unit: String,
        #[graphql(skip)]
        marker: PhantomData<(T, U)>,
    }

    impl<T: Unit + Send + Sync, U: Unit + Send + Sync> TypeName for RatioOutput<T, U> {
        fn type_name() -> Cow<'static, str> {
            format!("{}Per{}Ratio", T::NAME, U::NAME).into()
        }
    }

    impl<T, U> InputType for Ratio<T, U>
    where
        T: Unit + Send + Sync,
        U: Unit + Send + Sync,
        Ratio<T, U>: FromStr<Err = MetricError>,
    {
        type RawValueType = Self;

        fn type_name() -> Cow<'static, str> {
            format!("{}Per{}RatioInput", T::NAME, U::NAME).into()
        }

        fn create_type_info(registry: &mut Registry) -> String {
            let input_fields = IndexMap::from([
                graphql::create_meta_input_value::<Decimal>(registry, "value"),
                graphql::create_meta_input_value::<String>(registry, "numeratorUnit"),
                graphql::create_meta_input_value::<String>(registry, "denominatorUnit"),
            ]);
            graphql::create_input_type::<Self>(registry, input_fields)
        }

        fn parse(value: Option<Value>) -> InputValueResult<Self> {
            let input = RatioInput::parse(value).map_err(InputValueError::propagate)?;
            format!("{}{}/1{}", input.value, input.numerator_unit, input.denominator_unit)
                .parse()
                .map_err(InputValueError::custom)
        }

        fn to_value(&self) -> Value {
            RatioInput {
                value: self.get(),
                numerator_unit: self.numerator_unit().to_string(),
                denominator_unit: self.denominator_unit().to_string(),
            }
            .to_value()
        }

        fn as_raw_value(&self) -> Option<&Self::RawValueType> {
            Some(self)
        }
    }

    impl<T, U> OutputType for Ratio<T, U>
    where
        T: Unit + Send + Sync,
        U: Unit + Send + Sync,
    {
        fn type_name() -> Cow<'static, str> {
            <RatioOutput<T, U> as OutputType>::type_name()
        }

        fn create_type_info(registry: &mut Registry) -> String {
            RatioOutput::<T, U>::create_type_info(registry)
        }

        async fn resolve(&self, ctx: &ContextSelectionSet<'_>, field: &Positioned<Field>) -> ServerResult<Value> {
            RatioOutput::<T, U> {
                value: self.get(),
                numerator_unit: self.numerator_unit().to_string(),
                denominator_unit: self.denominator_unit().to_string(),
                marker: PhantomData,
            }
            .resolve(ctx, field)
            .await
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::PricePerTime;

        use super::*;

        #[test]
        fn graphql_input_name_includes_both_unit_names() {
            assert_eq!(<PricePerTime as InputType>::type_name(), "PricePerTimeRatioInput");
        }

        #[test]
        fn graphql_output_name_includes_both_unit_names() {
            assert_eq!(<PricePerTime as OutputType>::type_name(), "PricePerTimeRatio");
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    use super::*;

    impl<T, U> Serialize for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            <String as Serialize>::serialize(&self.to_string(), serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
        Ratio<T, U>: FromStr<Err = MetricError>,
    {
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
    use rust_decimal::dec;

    use crate::metric::{Mass, Power, Price, Time};

    use super::*;

    #[test]
    fn test_ratio_canonicalize() {
        for (expected_value, ratio) in [
            (
                dec!(5000) / dec!(120),
                Ratio::new(Mass::from_kilograms(dec!(5)), Time::from_hours(dec!(2))),
            ),
            (
                dec!(1000) / dec!(60),
                Ratio::new(Mass::from_kilograms(dec!(1)), Time::from_hours(dec!(1))),
            ),
            (
                dec!(48) / dec!(2880),
                Ratio::new(Mass::from_grams(dec!(48)), Time::from_days(dec!(2))),
            ),
            (
                dec!(17520) / dec!(525600),
                Ratio::new(Mass::from_kilograms(dec!(17.520)), Time::from_years(dec!(1))),
            ),
        ] {
            assert_eq!(expected_value, ratio.canonicalize().get());
        }
    }

    #[test]
    fn test_ratio_add() {
        let mut ratio = Ratio::new(Mass::from_kilograms(dec!(1)), Time::from_hours(dec!(1)));

        ratio += Ratio::new(Mass::from_grams(dec!(250)), Time::from_hours(dec!(2)));

        assert_eq!(dec!(1.125), ratio.get());
    }

    #[test]
    fn test_ratio_sub() {
        let mut ratio = Ratio::new(Mass::from_kilograms(dec!(1)), Time::from_days(dec!(1)));

        ratio -= Ratio::new(Mass::from_grams(dec!(10)), Time::from_hours(dec!(1)));

        assert_eq!(dec!(0.760), ratio.get());
    }

    #[test]
    fn test_ratio_mul_by_scalar() {
        let mut ratio = Ratio::new(Mass::from_kilograms(dec!(1)), Time::from_hours(dec!(1)));

        ratio *= dec!(2.5);

        assert_eq!(dec!(2.5), ratio.get());
    }

    #[test]
    fn test_ratio_div_by_scalar() {
        let mut ratio = Ratio::new(Mass::from_kilograms(dec!(5)), Time::from_hours(dec!(1)));

        ratio /= dec!(2);

        assert_eq!(dec!(2.5), ratio.get());
    }

    #[test]
    fn test_ratio_mul_by_metric() {
        let ratio = Ratio::new(Mass::from_kilograms(dec!(2)), Time::from_hours(dec!(1)));

        let metric = ratio * Time::from_hours(dec!(3));

        assert_eq!(Mass::from_kilograms(dec!(6)), metric);
    }

    #[test]
    fn test_ratio_mul() {
        let price_per_energy = Ratio::new(Price::from_euros(dec!(5)), Power::from_kilowatts(dec!(1)));
        let power_per_time = Ratio::new(Power::from_watts(dec!(500)), Time::from_hours(dec!(1)));

        let price_per_time = price_per_energy * power_per_time;

        assert_eq!(dec!(2.5), price_per_time.get());
    }

    #[test]
    fn test_ratio_div() {
        let price_per_time = Ratio::new(Price::from_euros(dec!(120)), Time::from_days(dec!(2)));
        let power_per_time = Ratio::new(Power::from_watts(dec!(100)), Time::from_hours(dec!(1)));

        let price_per_energy = price_per_time / power_per_time;

        assert_eq!(dec!(0.025), price_per_energy.get());
    }
}
