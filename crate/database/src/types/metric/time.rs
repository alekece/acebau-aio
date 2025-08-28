use std::str::FromStr;

use derive_more::{Display};

use super::{Metric, Unit, UnitError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Display)]
pub enum TimeUnit {
    #[display("y")]
    Year,
    #[display("d")]
    Day,
    #[display("h")]
    #[default]
    Hour,
}

impl Unit for TimeUnit {
    fn factor(&self) -> f32 {
        match self {
            TimeUnit::Year => 8760.,
            TimeUnit::Day => 24.,
            TimeUnit::Hour => 1.,
        }
    }
}

pub type Time = Metric<TimeUnit>;

impl Time {
    pub fn from_years(value: f32) -> Self {
        Self::new(value, TimeUnit::Year)
    }

    pub fn from_days(value: f32) -> Self {
        Self::new(value, TimeUnit::Day)
    }

    pub fn from_hours(value: f32) -> Self {
        Self::new(value, TimeUnit::Hour)
    }

    pub fn to_years(self) -> Self {
        self.convert_to(TimeUnit::Year)
    }

    pub fn to_days(self) -> Self {
        self.convert_to(TimeUnit::Day)
    }

    pub fn to_hours(self) -> Self {
        self.convert_to(TimeUnit::Hour)
    }
}

impl FromStr for TimeUnit {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "y" => Ok(TimeUnit::Year),
            "d" => Ok(TimeUnit::Day),
            "h" => Ok(TimeUnit::Hour),
            _ => Err(UnitError::Unknown { unit: s.to_string() }),
        }
    }
}
