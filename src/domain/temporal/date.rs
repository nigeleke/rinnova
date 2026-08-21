use dioxus_i18n::tid;
use jiff::Zoned;
use jiff::civil::Date as JiffDate;
use serde::{Deserialize, Serialize};

use crate::domain::{LogbookError, Period};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date(JiffDate);

impl Date {
    pub fn today() -> Self {
        Self(Zoned::now().date())
    }

    pub fn to_iso8601_string(&self) -> String {
        self.0.to_string()
    }

    pub fn parse_iso8601_str(value: &str) -> Result<Self, LogbookError> {
        use std::str::FromStr;
        let date = JiffDate::from_str(value)?;
        Ok(Self(date))
    }
}

impl std::ops::Add<Period> for Date {
    type Output = Date;

    fn add(self, rhs: Period) -> Self::Output {
        Self(self.0 + rhs.into_jiff())
    }
}

#[cfg(test)]
impl std::ops::Sub<Period> for Date {
    type Output = Date;

    fn sub(self, rhs: Period) -> Self::Output {
        Self(self.0 - rhs.into_jiff())
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day = format!("{:02}", self.0.day());
        let month = format!("{:02}", self.0.month());
        let year = format!("{:04}", self.0.year());
        let formatted = tid!("date", day: day, month: month, year: year);
        formatted.fmt(f)
    }
}
