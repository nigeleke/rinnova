use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct SupplyCount(usize);

impl SupplyCount {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
}

impl From<usize> for SupplyCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for SupplyCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl std::ops::AddAssign for SupplyCount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::ops::Sub for SupplyCount {
    type Output = SupplyCount;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl std::iter::Sum for SupplyCount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|i| i.0).sum())
    }
}

impl std::fmt::Display for SupplyCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
