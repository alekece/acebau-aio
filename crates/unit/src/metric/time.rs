use strum::{Display, EnumString};

use super::{Metric, Unit};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum TimeUnit {
    #[strum(serialize = "y")]
    Year,
    #[strum(serialize = "d")]
    Day,
    #[strum(serialize = "h")]
    #[default]
    Hour,
}

impl Unit for TimeUnit {
    fn factor(&self) -> f32 {
        match self {
            Self::Year => 8760.,
            Self::Day => 24.,
            Self::Hour => 1.,
        }
    }
}

pub type Time = Metric<TimeUnit>;

impl Time {
    pub fn from_years(value: f32) -> Self {
        Self::with_unit(value, TimeUnit::Year)
    }

    pub fn from_days(value: f32) -> Self {
        Self::with_unit(value, TimeUnit::Day)
    }

    pub fn from_hours(value: f32) -> Self {
        Self::with_unit(value, TimeUnit::Hour)
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
