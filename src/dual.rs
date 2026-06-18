use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};

use num::Float;

#[derive(Debug, Clone, Copy)]
pub struct Dual<T: Float> {
    pub real: T,
    pub dual: T,
}

pub trait DualComp: Float + Default + Display + FromStr {}
impl<T: Float + Default + Display + FromStr> DualComp for T {}

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
            dual: (self.dual * rhs.real - self.real * rhs.dual) / (rhs.real * rhs.real),
        }
    }
}

impl<T: DualComp> Neg for Dual<T> {
    type Output = Dual<T>;
    fn neg(self) -> Self::Output {
        Dual {
            real: -self.real,
            dual: -self.dual,
        }
    }
}

impl<T: DualComp> Display for Dual<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}\u{03B5}", self.real, self.dual)
    }
}