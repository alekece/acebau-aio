use core::num;
use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use ::serde::de::{self, value};
use derive_more::Display;
use snafu::Snafu;

use crate::{
    metric::{Metric, MetricError},
    unit::{Unit, UnitError},
};

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum RatioError {
    #[snafu(display("Invalid format, expected <metric>/<unit>, got '{input}'"))]
    InvalidFormat { input: String },
    #[snafu(display("Metric error: {source}"))]
    MetricError { source: MetricError },
    #[snafu(display("Unit error: {source}"))]
    UnitError { source: UnitError },
}

#[derive(Debug, Copy, Clone, Display)]
#[display("{value}{numerator}/{denominator}")]
pub struct Ratio<T: Unit, U: Unit> {
    value: f32,
    numerator: T,
    denominator: U,
}

impl<T: Unit, U: Unit> Ratio<T, U> {
    pub fn new(value: f32, numerator: T, denominator: U) -> Self {
        Self {
            value,
            numerator,
            denominator,
        }
    }

    pub fn convert_to(self, numerator: T, denominator: U) -> Self {
        let numerator_scale = self.numerator.factor() / numerator.factor();
        let denominator_scale = self.denominator.factor() / denominator.factor();

        Self {
            value: self.value * numerator_scale * denominator_scale,
            numerator,
            denominator,
        }
    }
}

impl<T: Unit, U: Unit> AddAssign for Ratio<T, U> {
    fn add_assign(&mut self, other: Self) {
        self.value += other.convert_to(self.numerator, self.denominator).value;
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
        self.value -= other.convert_to(self.numerator, self.denominator).value;
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
        self.value *= value;

        self
    }
}

impl<T: Unit, U: Unit> MulAssign<f32> for Ratio<T, U> {
    fn mul_assign(&mut self, value: f32) {
        self.value *= value;
    }
}

impl<T: Unit, U: Unit> Div<f32> for Ratio<T, U> {
    type Output = Self;

    fn div(mut self, value: f32) -> Self::Output {
        self.value /= value;

        self
    }
}

impl<T: Unit, U: Unit> DivAssign<f32> for Ratio<T, U> {
    fn div_assign(&mut self, value: f32) {
        self.value /= value;
    }
}

impl<T, U> Mul<Metric<U>> for Ratio<T, U>
where
    T: Unit,
    U: Unit,
{
    type Output = Metric<T>;

    fn mul(self, metric: Metric<U>) -> Self::Output {
        Metric::with_unit(self.value * metric.convert_to(self.denominator).get(), self.numerator)
    }
}

impl<T: Unit, U: Unit, V: Unit> Mul<Ratio<U, V>> for Ratio<T, U> {
    type Output = Ratio<T, V>;

    fn mul(self, other: Ratio<U, V>) -> Self::Output {
        Ratio::new(
            self.value * other.convert_to(self.denominator, other.denominator).value,
            self.numerator,
            other.denominator,
        )
    }
}

impl<T: Unit, U: Unit, V: Unit> Div<Ratio<U, V>> for Ratio<T, V> {
    type Output = Ratio<T, U>;

    fn div(self, other: Ratio<U, V>) -> Self::Output {
        Ratio::new(
            self.value / other.convert_to(other.numerator, self.denominator).value,
            self.numerator,
            other.numerator,
        )
    }
}

/*
impl<T: Unit, U: Unit, V: Unit> Div<Ratio<T, V>> for Ratio<T, U> {
    type Output = Ratio<V, U>;

    fn div(self, other: Ratio<T, V>) -> Self::Output {
        Ratio::new(
            self.convert_to(other.numerator, self.denominator).value / other.value,
            other.denominator,
            self.denominator,
        )
    }
}
*/

impl<T, U> FromStr for Ratio<T, U>
where
    T: Unit + FromStr<Err = UnitError>,
    U: Unit + FromStr<Err = UnitError>,
{
    type Err = RatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((metric, unit)) = s.split_once("/") else {
            return Err(RatioError::InvalidFormat { input: s.to_string() });
        };

        let metric: Metric<T> = metric.parse().map_err(|e| RatioError::MetricError { source: e })?;
        let unit = unit.parse().map_err(|e| RatioError::UnitError { source: e })?;

        Ok(Self::new(metric.get(), metric.unit(), unit))
    }
}

#[cfg(feature = "sqlx")]
mod sqlx {
    use ::sqlx::{Database, Decode, Encode, Postgres, Type, encode::IsNull, error::BoxDynError, postgres::PgTypeInfo};

    use super::*;

    /*
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
            <f32 as Encode<'_, Postgres>>::encode_by_ref(
                &f32::from(self.metric.convert_to(T::default()) * U::default().factor() / self.unit.factor()),
                buf,
            )
        }
    }

    impl<T, U> Decode<'_, Postgres> for Ratio<T, U>
    where
        T: Unit,
        U: Unit,
        Ratio<T, U>: From<f32>,
    {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            Ok(Self {
                metric: Metric::with_unit(<f32 as Decode<'_, Postgres>>::decode(value)?, T::default()),
                unit: U::default(),
            })
        }
    }

    */
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
        T: Unit + FromStr<Err = UnitError>,
        U: Unit + FromStr<Err = UnitError>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Self::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::types::{Mass, TimeUnit};

//     use super::*;

//     #[test]
//     fn test_rate_into_f32() {
//         let rate: Rate<Mass, TimeUnit> = Rate {
//             value: Mass::from_kilograms(1.),
//             unit: TimeUnit::Hour,
//         };

//         let value: f32 = rate.into();
//         assert_eq!(value, 1000.0);
//     }
// }
