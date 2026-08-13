use std::str::FromStr;

use darling::{Error, FromMeta};
use syn::Lit;

#[derive(Clone, Copy)]
pub struct Factor {
    lo: u32,
    mid: u32,
    hi: u32,
    scale: u32,
}

impl Factor {
    pub fn parts(&self) -> (u32, u32, u32, u32) {
        (self.lo, self.mid, self.hi, self.scale)
    }
}

impl FromStr for Factor {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.contains(['e', 'E']) {
            return Err(Error::custom("unit factor must use plain decimal notation"));
        }

        let value = value.replace('_', "");
        let Some((whole, fraction)) = value.split_once('.').map_or_else(
            || Some((value.as_str(), "")),
            |parts| (!parts.1.contains('.')).then_some(parts),
        ) else {
            return Err(Error::custom("unit factor must be a valid decimal literal"));
        };

        if whole.is_empty()
            || !whole.bytes().all(|byte| byte.is_ascii_digit())
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(Error::custom("unit factor must be a valid decimal literal"));
        }

        let scale = u32::try_from(fraction.len()).map_err(|_| Error::custom("unit factor has too much precision"))?;
        if scale > 28 {
            return Err(Error::custom("unit factor cannot exceed 28 decimal places"));
        }

        let mantissa = format!("{whole}{fraction}")
            .parse::<u128>()
            .map_err(|_| Error::custom("unit factor exceeds Decimal's 96-bit range"))?;
        if mantissa == 0 {
            return Err(Error::custom("unit factor must be greater than zero"));
        }
        if mantissa >= 1_u128 << 96 {
            return Err(Error::custom("unit factor exceeds Decimal's 96-bit range"));
        }

        Ok(Self {
            lo: mantissa as u32,
            mid: (mantissa >> 32) as u32,
            hi: (mantissa >> 64) as u32,
            scale,
        })
    }
}

impl FromMeta for Factor {
    fn from_value(value: &Lit) -> Result<Self, Error> {
        let result = match value {
            Lit::Int(value) if value.suffix().is_empty() => value.base10_digits().parse(),
            Lit::Float(value) if value.suffix().is_empty() => value.to_string().parse(),
            Lit::Int(_) | Lit::Float(_) => Err(Error::custom("unit factor cannot have a numeric suffix")),
            _ => Err(Error::unexpected_lit_type(value)),
        };
        result.map_err(|error| error.with_span(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Factor;

    #[test]
    fn parses_exact_decimal_factors() {
        assert_eq!((125, 0, 0, 3), "0.125".parse::<Factor>().unwrap().parts());
        assert_eq!((525_600, 0, 0, 0), "525_600".parse::<Factor>().unwrap().parts());
    }

    #[test]
    fn rejects_invalid_decimal_factors() {
        for factor in ["0", "0.0", "1e-3", "1E3", "1.2.3", "0.12345678901234567890123456789"] {
            assert!(factor.parse::<Factor>().is_err(), "{factor} should be rejected");
        }
    }
}
