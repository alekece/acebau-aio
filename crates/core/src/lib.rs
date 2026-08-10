#![allow(clippy::pedantic)]

use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageSize {
    Ten,
    TwentyFive,
    Fifty,
    OneHundred,
}

impl PageSize {
    pub const DEFAULT: Self = Self::Ten;

    pub const fn get(self) -> u32 {
        match self {
            Self::Ten => 10,
            Self::TwentyFive => 25,
            Self::Fifty => 50,
            Self::OneHundred => 100,
        }
    }
}

impl TryFrom<u32> for PageSize {
    type Error = InvalidPageSize;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(Self::Ten),
            25 => Ok(Self::TwentyFive),
            50 => Ok(Self::Fifty),
            100 => Ok(Self::OneHundred),
            _ => Err(InvalidPageSize(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidPageSize(pub u32);

impl std::fmt::Display for InvalidPageSize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "page size {} is invalid; expected 10, 25, 50, or 100",
            self.0
        )
    }
}

impl std::error::Error for InvalidPageSize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    page: NonZeroU32,
    size: PageSize,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: NonZeroU32::MIN,
            size: PageSize::DEFAULT,
        }
    }
}

impl PageRequest {
    pub fn new(page: NonZeroU32, size: PageSize) -> Self {
        Self { page, size }
    }

    pub const fn limit(self) -> u32 {
        self.size.get()
    }

    pub const fn offset(self) -> u32 {
        (self.page.get() - 1) * self.size.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{PageRequest, PageSize};
    use std::num::NonZeroU32;

    #[test]
    fn page_request_calculates_limit_and_offset() {
        let page = PageRequest::new(NonZeroU32::new(3).unwrap(), PageSize::TwentyFive);
        assert_eq!(page.limit(), 25);
        assert_eq!(page.offset(), 50);
    }
}
