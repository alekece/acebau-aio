use std::{
    ops::{Add, AddAssign, Deref, DerefMut},
    str::FromStr,
};

use derive_more::Display;
use snafu::Snafu;
use sqlx::{encode::IsNull, postgres::PgTypeInfo, Database, Decode, Encode, Postgres, Type};

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum QuantityError {
    #[snafu(display("Unknown unit '{unit}'"))]
    UnknownUnit { unit: String },
    #[snafu(display("Quantity value must be a floating point number, got '{value}'"))]
    NotANumber { value: String },
    #[snafu(display("Invalid format for quantity, expected '<value> <unit>', got '{quantity}'"))]
    InvalidFormat { quantity: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Unit {
    Kilogram,
    Gram,
}

/// `Quantity`, a type representing a quantity expressed in different units.
/// It offers conversion and arithmetic operations between units such as kilograms and grams and
/// parsing from strings.
#[derive(Debug, Clone, Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Quantity {
    /// Represents a quantity in kilograms.
    #[display("{_0}kg")]
    Kilogram(f32),
    /// Represents a quantity in grams.
    #[display("{_0}g")]
    Gram(f32),
}

impl Quantity {
    pub fn to_gram(&self) -> Self {
        self.convert_to(Unit::Gram)
    }

    pub fn to_kilogram(&self) -> Self {
        self.convert_to(Unit::Kilogram)
    }

    pub fn convert_to(&self, unit: Unit) -> Self {
        match (self, unit) {
            (Self::Gram(value), Unit::Kilogram) => Self::Kilogram(value / 1000.0),
            (Self::Kilogram(value), Unit::Gram) => Self::Gram(value * 1000.0),
            _ => self.clone(),
        }
    }

    pub fn unit(&self) -> Unit {
        match self {
            Self::Kilogram(_) => Unit::Kilogram,
            Self::Gram(_) => Unit::Gram,
        }
    }
}

impl Deref for Quantity {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Kilogram(value) => value,
            Self::Gram(value) => value,
        }
    }
}

impl DerefMut for Quantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Kilogram(value) => value,
            Self::Gram(value) => value,
        }
    }
}

impl Add for Quantity {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        let unit = self.unit();

        self.add_assign(other.convert_to(unit));

        self
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, other: Self) {
        let unit = self.unit();

        self.deref_mut().add_assign(other.convert_to(unit).deref());
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        *self.to_gram() == *other.to_gram()
    }
}

impl Type<Postgres> for Quantity {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("number")
    }
}

impl Encode<'_, Postgres> for Quantity {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'_>,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        <f32 as Encode<'_, Postgres>>::encode_by_ref(&*self.to_gram(), buf)
    }
}

impl Decode<'_, Postgres> for Quantity {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Self::Gram(<f32 as Decode<'_, Postgres>>::decode(value)?))
    }
}

impl FromStr for Quantity {
    type Err = QuantityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, unit) = match s.trim_start().split_once(' ') {
            Some((value, unit)) => (value, unit),
            _ => s
                .find(|c: char| c.is_alphabetic())
                .or(Some(0))
                .map(|index| (&s[..index], &s[index..]))
                .unwrap(),
        };

        let value = value.trim();

        if value.is_empty() {
            return Err(QuantityError::InvalidFormat {
                quantity: s.to_string(),
            });
        }

        let value = value.parse::<f32>().map_err(|_| QuantityError::NotANumber {
            value: value.to_string(),
        })?;

        let unit = unit.trim().to_lowercase();

        match unit.as_str() {
            "kg" => Ok(Quantity::Kilogram(value)),
            "g" => Ok(Quantity::Gram(value)),
            _ => Err(QuantityError::UnknownUnit { unit }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() -> Result<(), QuantityError> {
        assert_eq!(Quantity::from_str("1.5kg")?, Quantity::Kilogram(1.5));
        assert_eq!(Quantity::from_str("500g")?, Quantity::Gram(500.));
        assert_eq!(Quantity::from_str("5 kg")?, Quantity::Kilogram(5.));
        assert_eq!(Quantity::from_str("  30.2  g ")?, Quantity::Gram(30.2));

        Ok(())
    }

    #[test]
    fn test_from_str_invalid_format() -> Result<(), QuantityError> {
        for quantity in ["10", "", "      g", "   "].into_iter() {
            let quantity = quantity.to_string();
            let result = Quantity::from_str(&quantity);

            assert!(result.is_err());
            assert_eq!(result.err().unwrap(), QuantityError::InvalidFormat { quantity });
        }

        Ok(())
    }

    #[test]
    fn test_from_str_unknown_unit() {
        let result = Quantity::from_str("10.5xyz");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            QuantityError::UnknownUnit {
                unit: "xyz".to_string()
            }
        );
    }

    #[test]
    fn test_from_str_not_a_number() {
        let result = Quantity::from_str("ten kg");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            QuantityError::NotANumber {
                value: "ten".to_string()
            }
        );
    }

    #[test]
    fn test_add_gram_to_kilogram() {
        assert_eq!(Quantity::Kilogram(1.5), Quantity::Kilogram(1.0) + Quantity::Gram(500.0));
        assert_eq!(
            Quantity::Kilogram(0.9),
            Quantity::Kilogram(1.0) + Quantity::Gram(-100.0)
        );
    }

    #[test]
    fn test_add_kilogram_to_gram() {
        assert_eq!(Quantity::Gram(2500.0), Quantity::Gram(1000.0) + Quantity::Kilogram(1.5));
        assert_eq!(Quantity::Gram(900.0), Quantity::Gram(1000.0) + Quantity::Kilogram(-0.1));
    }
}
