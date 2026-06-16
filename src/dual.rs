use std::ops::{Add, Div, Mul, Sub};

use num::Float;

pub struct Dual<T: Float> {
    real: T,
    dual: T,
}

impl<T: Float> Add for Dual<T> {
    type Output = Dual<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real + rhs.real,
            dual: self.dual + rhs.dual,
        }
    }
}

impl<T: Float> Sub for Dual<T> {
    type Output = Dual<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real - rhs.real,
            dual: self.dual - rhs.dual,
        }
    }
}

impl<T: Float> Mul for Dual<T> {
    type Output = Dual<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real * rhs.real,
            dual: self.real * rhs.dual + self.dual * rhs.real,
        }
    }
}

impl<T: Float> Div for Dual<T> {
    type Output = Dual<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Dual {
            real: self.real / rhs.real,
            dual: (self.dual * rhs.real - self.real * rhs.dual) / (rhs.dual * rhs.dual),
        }
    }
}