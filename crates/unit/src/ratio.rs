use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use derive_more::Display;

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
            denominator: Metric::with_unit(1.0, denominator.unit()),
        }
    }

    pub fn with_units(value: f32, numerator: T, denominator: U) -> Self {
        Self {
            numerator: Metric::with_unit(value, numerator),
            denominator: Metric::with_unit(1.0, denominator),
        }
    }

    pub fn get(&self) -> f32 {
        self.numerator.value()
    }

    pub fn convert_to(self, numerator: T, denominator: U) -> Self {
        Self::new(
            self.numerator.convert_to(numerator),
            self.denominator.convert_to(denominator),
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

impl<T: Unit, U: Unit> Mul<f32> for Ratio<T, U> {
    type Output = Self;

    fn mul(mut self, value: f32) -> Self::Output {
        self.numerator *= value;

        self
    }
}

impl<T: Unit, U: Unit> MulAssign<f32> for Ratio<T, U> {
    fn mul_assign(&mut self, value: f32) {
        self.numerator *= value;
    }
}

impl<T: Unit, U: Unit> Div<f32> for Ratio<T, U> {
    type Output = Self;

    fn div(mut self, value: f32) -> Self::Output {
        self.numerator /= value;

        self
    }
}

impl<T: Unit, U: Unit> DivAssign<f32> for Ratio<T, U> {
    fn div_assign(&mut self, value: f32) {
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
            Metric::with_unit(1.0, other.numerator.unit()),
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
    use ::sqlx::{encode::IsNull, error::BoxDynError, postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

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
            <f32 as Encode<'_, Postgres>>::encode_by_ref(&self.canonicalize().numerator.value(), buf)
        }
    }

    impl<T, U> Decode<'_, Postgres> for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
    {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            Ok(Self::new(
                Metric::with_unit(<f32 as Decode<'_, Postgres>>::decode(value)?, T::default()),
                Metric::with_unit(1.0, U::default()),
            ))
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

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
    use crate::metric::{Energy, Mass, Price, Time};

    use super::*;

    #[test]
    fn test_ratio_canonicalize() {
        for (expected_value, ratio) in [
            (2500.0, Ratio::new(Mass::from_kilograms(5.0), Time::from_hours(2.0))),
            (1000.0, Ratio::new(Mass::from_kilograms(1.0), Time::from_hours(1.0))),
            (1.0, Ratio::new(Mass::from_grams(48.0), Time::from_days(2.0))),
            (2.0, Ratio::new(Mass::from_kilograms(17.520), Time::from_years(1.0))),
        ] {
            assert_eq!(expected_value, ratio.canonicalize().get());
        }
    }

    #[test]
    fn test_ratio_add() {
        let mut ratio = Ratio::new(Mass::from_kilograms(1.0), Time::from_hours(1.0));

        ratio += Ratio::new(Mass::from_grams(250.0), Time::from_hours(2.0));

        assert_eq!(1.125, ratio.get());
    }

    #[test]
    fn test_ratio_sub() {
        let mut ratio = Ratio::new(Mass::from_kilograms(1.0), Time::from_days(1.0));

        ratio -= Ratio::new(Mass::from_grams(10.0), Time::from_hours(1.0));

        assert_eq!(0.760, ratio.get());
    }

    #[test]
    fn test_ratio_mul_by_scalar() {
        let mut ratio = Ratio::new(Mass::from_kilograms(1.0), Time::from_hours(1.0));

        ratio *= 2.5;

        assert_eq!(2.5, ratio.get());
    }

    #[test]
    fn test_ratio_div_by_scalar() {
        let mut ratio = Ratio::new(Mass::from_kilograms(5.0), Time::from_hours(1.0));

        ratio /= 2.0;

        assert_eq!(2.5, ratio.get());
    }

    #[test]
    fn test_ratio_mul_by_metric() {
        let ratio = Ratio::new(Mass::from_kilograms(2.0), Time::from_hours(1.0));

        let metric = ratio * Time::from_hours(3.0);

        assert_eq!(Mass::from_kilograms(6.0), metric);
    }

    #[test]
    fn test_ratio_mul() {
        let price_per_energy = Ratio::new(Price::new(5.0), Energy::from_kilowatts(1.0));
        let energy_per_time = Ratio::new(Energy::from_watts(500.0), Time::from_hours(1.0));

        let price_per_time = price_per_energy * energy_per_time;

        assert_eq!(2.5, price_per_time.get());
    }

    #[test]
    fn test_ratio_div() {
        let price_per_time = Ratio::new(Price::new(120.0), Time::from_days(2.0));
        let energy_per_time = Ratio::new(Energy::from_watts(100.0), Time::from_hours(1.0));

        let price_per_energy = price_per_time / energy_per_time;

        assert_eq!(0.025, price_per_energy.get());
    }
}
