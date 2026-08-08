use dioxus_i18n::tid;
use jiff::civil::Date as JiffDate;
use jiff::{Span, Zoned};
use serde::{Deserialize, Serialize};

use crate::domain::LogbookError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date(JiffDate);

impl Date {
    pub fn today() -> Self {
        Self(Zoned::now().date())
    }

    pub fn plus_days(self, value: i64) -> Self {
        Self(self.0 + Span::new().days(value))
    }

    pub fn less_days(self, value: i64) -> Self {
        Self(self.0 - Span::new().days(value))
    }

    pub fn plus_months(self, value: i64) -> Self {
        Self(self.0 + Span::new().months(value))
    }

    pub fn less_months(self, value: i64) -> Self {
        Self(self.0 - Span::new().months(value))
    }

    pub fn plus_years(self, value: i64) -> Self {
        Self(self.0 + Span::new().years(value))
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

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day = format!("{:02}", self.0.day());
        let month = format!("{:02}", self.0.month());
        let year = format!("{:04}", self.0.year());
        let formatted = tid!("date", day: day, month: month, year: year);
        formatted.fmt(f)
    }
}
