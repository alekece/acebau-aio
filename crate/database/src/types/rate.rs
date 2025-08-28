use std::{ops::Mul, str::FromStr};

use derive_more::{Deref, DerefMut, Display};
use snafu::Snafu;
use sqlx::{encode::IsNull, error::BoxDynError, postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

use super::{Metric, MetricError, Unit, UnitError};

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum RateError {
    #[snafu(display("Invalid format, expected <metric>/<unit>, got '{input}'"))]
    InvalidFormat { input: String },
    #[snafu(display("Metric error: {source}"))]
    MetricError { source: MetricError },
    #[snafu(display("Unit error: {source}"))]
    UnitError { source: UnitError },
}

#[derive(Debug, Copy, Clone, Deref, DerefMut, Display)]
#[display("{metric}/{unit}")]
pub struct Rate<T: Unit, U: Unit> {
    #[deref]
    #[deref_mut]
    metric: Metric<T>,
    unit: U,
}

impl<T, U> Mul<Metric<U>> for Rate<T, U>
where
    T: Unit,
    U: Unit,
{
    type Output = Metric<T>;

    fn mul(self, metric: Metric<U>) -> Self::Output {
        self.metric * metric.convert_to(self.unit).get()
    }
}

impl<T, U> From<f32> for Rate<T, U>
where
    T: Unit,
    U: Unit + Default,
{
    fn from(value: f32) -> Self {
        Self {
            metric: Metric::<T>::from(value),
            unit: U::default(),
        }
    }
}

impl<T, U> From<Rate<T, U>> for f32
where
    T: Unit,
    U: Unit + Default,
    f32: From<Metric<T>>,
{
    fn from(rate: Rate<T, U>) -> f32 {
        rate.metric.get() * U::default().factor() / rate.unit.factor()
    }
}

impl<T: Unit, U: Unit> Type<Postgres> for Rate<T, U> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("numeric")
    }
}

impl<T, U> Encode<'_, Postgres> for Rate<T, U>
where
    T: Unit,
    U: Unit,
    f32: From<Rate<T, U>>,
{
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
        <f32 as Encode<'_, Postgres>>::encode_by_ref(&f32::from(*self), buf)
    }
}

impl<T, U> Decode<'_, Postgres> for Rate<T, U>
where
    T: Unit,
    U: Unit,
    Rate<T, U>: From<f32>,
{
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(Self::from(<f32 as Decode<'_, Postgres>>::decode(value)?))
    }
}

impl<T, U> FromStr for Rate<T, U>
where
    T: Unit + FromStr<Err = UnitError>,
    U: Unit + FromStr<Err = UnitError>,
{
    type Err = RateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((metric, unit)) = s.split_once("/") else {
            return Err(RateError::InvalidFormat { input: s.to_string() });
        };

        let metric = metric.parse().map_err(|e| RateError::MetricError { source: e })?;
        let unit = unit.parse().map_err(|e| RateError::UnitError { source: e })?;

        Ok(Self { metric, unit })
    }
}

#[cfg(feature = "serde")]
mod serde {
    use ::serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    impl<T, U> Serialize for Rate<T, U>
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

    impl<'de, T, U> Deserialize<'de> for Rate<T, U>
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
