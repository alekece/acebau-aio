use snafu::{ensure, Snafu};
use sqlx::{encode::IsNull, error::BoxDynError, postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

#[derive(Debug, PartialEq, Snafu)]
pub enum PercentageError {
    #[snafu(display("Percentage must be a number between 0 and 100: got {value}"))]
    OutOfBounds { value: f32 },
}

/// `PercentageDecimal`, a type representing a percentage value for any floating point type.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Percentage(f32);

impl Percentage {
    pub fn try_new(value: f32) -> Result<Self, PercentageError> {
        ensure!((0.0..=100.0).contains(&value), OutOfBoundsSnafu { value });

        Ok(Self(value / 100.))
    }

    pub fn to_float(&self) -> f32 {
        self.0
    }

    /// Apply the percentage to a value.
    pub fn apply_to(&self, value: f32) -> f32 {
        value * self.0
    }

    /// Scale down a value by the percentage.
    pub fn scale_down(&self, value: f32) -> f32 {
        value / (1. + self.0)
    }

    /// Scale up a value by the percentage.
    pub fn scale_up(&self, value: f32) -> f32 {
        value * (1. + self.0)
    }
}

impl Type<Postgres> for Percentage {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("float4")
    }
}

impl Encode<'_, Postgres> for Percentage {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
        <f32 as Encode<'_, Postgres>>::encode_by_ref(&self.0, buf)
    }
}

impl Decode<'_, Postgres> for Percentage {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(Self(<f32 as Decode<'_, Postgres>>::decode(value)?))
    }
}
