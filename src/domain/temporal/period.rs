use jiff::{Span, ToSpan};

pub struct Period(Span);

impl Period {
    #[inline]
    pub fn script_expiry_warning() -> Self {
        Self(14.days())
    }

    #[inline]
    pub fn retention_period() -> Self {
        Self(6.months())
    }

    #[inline]
    pub fn one_year() -> Self {
        Self(1.year())
    }

    #[inline]
    pub fn into_jiff(self) -> Span {
        self.0
    }
}

#[cfg(test)]
impl From<Span> for Period {
    fn from(value: Span) -> Self {
        Self(value)
    }
}
