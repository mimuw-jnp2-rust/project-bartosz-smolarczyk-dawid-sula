use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use ordered_float::NotNan;
use serde::{Serialize, Deserialize};

use super::InnerValue;

#[derive(Copy, Clone, Debug)]
pub struct Volume {
    value: NotNan<InnerValue>,
}

impl Volume {
    pub fn min() -> Volume {
        Volume::new(InnerValue::MIN)
    }
    pub fn max() -> Volume {
        Volume::new(InnerValue::MAX)
    }
    pub fn zero() -> Volume {
        Volume::new(0.)
    }

    pub fn new(value: InnerValue) -> Volume {
        Volume::from_float(value)
    }

    pub fn from_float(value: InnerValue) -> Volume {
        Volume {
            value: NotNan::new(value).unwrap(),
        }
    }

    pub fn from_notnan(value: NotNan<f64>) -> Volume {
        Volume { value }
    }

    pub fn float(&self) -> InnerValue {
        self.value.into_inner()
    }

    pub fn notnan(&self) -> NotNan<InnerValue> {
        self.value
    }

    fn eq(&self, other: &Self) -> bool {
        self.notnan() == other.notnan()
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.notnan().cmp(&other.notnan())
    }

    pub fn abs(&self) -> Self {
        if self.float() > 0. {
            *self
        } else {
            -*self
        }
    }
}

impl AddAssign for Volume {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.notnan();
    }
}

impl Add for Volume {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.add_assign(rhs);
        res
    }
}

impl SubAssign for Volume {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.notnan();
    }
}

impl Sub for Volume {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.sub_assign(rhs);
        res
    }
}

impl Neg for Volume {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_notnan(-self.notnan())
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Volume {}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Volume {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}

impl Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.float().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        InnerValue::deserialize(deserializer).map(|x| Volume::new(x))
    }
}
