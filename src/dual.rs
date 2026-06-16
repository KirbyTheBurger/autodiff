use std::ops::{Add, Div, Mul, Sub};

use num::Float;

pub struct Dual<T: Float> {
    real: T,
    dual: T,
}

pub trait DualComp: Float + Default {}
impl<T: Float + Default> DualComp for T {}

impl<T: DualComp> Dual<T> {
    pub fn new(real: T, dual: T) -> Dual<T> {
        Dual {
            real,
            dual,
        }
    }
}

impl<T: DualComp> From<T> for Dual<T> {
    fn from(value: T) -> Self {
        Dual {
            real: value,
            dual: T::default(),
        }
    }
}

impl<T: DualComp> Add for Dual<T> {
    type Output = Dual<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real + rhs.real,
            dual: self.dual + rhs.dual,
        }
    }
}

impl<T: DualComp> Sub for Dual<T> {
    type Output = Dual<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}

impl<T: DualComp> Mul for Dual<T> {
    type Output = Dual<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real * rhs.real,
            dual: self.real * rhs.dual + self.dual * rhs.real,
        }
    }
}

impl<T: DualComp> Div for Dual<T> {
    type Output = Dual<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real / rhs.real,
            dual: (self.dual * rhs.real - self.real * rhs.dual) / (rhs.dual * rhs.dual),
        }
    }
}