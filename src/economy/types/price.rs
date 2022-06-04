use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use ordered_float::NotNan;

use super::InnerValue;

#[derive(Copy, Clone, Debug)]
pub struct Price {
    value: NotNan<InnerValue>,
}

impl Price {
    pub fn min() -> Price {
        Price::new(InnerValue::MIN)
    }
    pub fn max() -> Price {
        Price::new(InnerValue::MAX)
    }
    pub fn zero() -> Price {
        Price::new(0.)
    }

    pub fn new(value: InnerValue) -> Price {
        Price::from_float(value)
    }

    pub fn from_float(value: InnerValue) -> Price {
        Price {
            value: NotNan::new(value).unwrap(),
        }
    }

    pub fn from_notnan(value: NotNan<f64>) -> Price {
        Price { value }
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

impl AddAssign for Price {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.notnan();
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.add_assign(rhs);
        res
    }
}

impl SubAssign for Price {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.notnan();
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res.sub_assign(rhs);
        res
    }
}

impl Neg for Price {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_notnan(-self.notnan())
    }
}

impl Mul<InnerValue> for Price {
    type Output = Self;

    fn mul(self, rhs: InnerValue) -> Self::Output {
        Price::from_notnan(self.notnan() * rhs)
    }
}

impl Div<InnerValue> for Price {
    type Output = Self;

    fn div(self, rhs: InnerValue) -> Self::Output {
        Price::from_notnan(self.notnan() / rhs)
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Price {}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}
