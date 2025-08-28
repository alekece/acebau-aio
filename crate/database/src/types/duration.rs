use derive_more::From;
use snafu::{ResultExt, Snafu};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{types::PgInterval, PgTypeInfo},
    Database, Decode, Encode, Postgres, Type,
};

#[derive(Debug, Snafu)]
pub enum DurationError {
    #[snafu(display("Cannot decode duration: {source}"))]
    Decode { source: BoxDynError },
    #[snafu(display("Cannot encode duration: {source}"))]
    Encode { source: BoxDynError },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Duration(chrono::Duration);

impl Duration {
    pub fn from_seconds(seconds: i64) -> Self {
        Self(chrono::Duration::seconds(seconds))
    }

    pub fn as_hours(&self) -> f32 {
        self.0.num_minutes() as f32 / 60.
    }

    pub fn as_years(&self) -> f32 {
        self.0.num_days() as f32 / 365.
    }
}

impl Type<Postgres> for Duration {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("interval")
    }
}

impl Encode<'_, Postgres> for Duration {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
        PgInterval::try_from(self.0).context(EncodeSnafu)?.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Duration {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        let interval = <PgInterval as Decode<'_, Postgres>>::decode(value).context(DecodeSnafu)?;

        let month_as_seconds = (interval.months as i64) * 30 * 24 * 60 * 60;
        let day_as_seconds = (interval.days as i64) * 24 * 60 * 60;
        let microsecond_as_seconds = interval.microseconds / 1_000_000;

        Ok(Self::from_seconds(
            month_as_seconds + day_as_seconds + microsecond_as_seconds,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_hour() {
        for (duration, expected_hours) in [
            (Duration::from_seconds(3600), 1.0),
            (Duration::from_seconds(7200), 2.0),
            (Duration::from_seconds(1800), 0.5),
            (Duration::from_seconds(0), 0.0),
        ] {
            assert_eq!(duration.as_hours(), expected_hours);
        }
    }
}
