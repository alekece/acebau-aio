use strum::{Display, EnumString};

use super::{Metric, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum Time {
    #[strum(serialize = "y")]
    Year,
    #[strum(serialize = "d")]
    Day,
    #[strum(serialize = "h")]
    #[default]
    Hour,
}

impl Unit for Time {
    fn factor(&self) -> f32 {
        match self {
            Self::Year => 8760.,
            Self::Day => 24.,
            Self::Hour => 1.,
        }
    }
}

impl Metric<Time> {
    pub fn from_years(value: f32) -> Self {
        Self::with_unit(value, Time::Year)
    }

    pub fn from_days(value: f32) -> Self {
        Self::with_unit(value, Time::Day)
    }

    pub fn from_hours(value: f32) -> Self {
        Self::with_unit(value, Time::Hour)
    }

    pub fn to_years(self) -> Self {
        self.convert_to(Time::Year)
    }

    pub fn to_days(self) -> Self {
        self.convert_to(Time::Day)
    }

    pub fn to_hours(self) -> Self {
        self.convert_to(Time::Hour)
    }
}
