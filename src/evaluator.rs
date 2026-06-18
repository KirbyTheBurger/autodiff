use crate::{dual::{Dual, DualComp}, parser::Expr};

pub struct Evaluator<T: DualComp> {
    var: Dual<T>,
}

impl<T: DualComp> Evaluator<T> {
    pub fn new(pos: T) -> Evaluator<T> {
        Evaluator {
            var: Dual::new(pos, T::one()),
        }
    }

    pub fn evaluate(&self, expr: Expr<T>) -> Dual<T> {
        match expr {
            Expr::Const(a) => a,
            Expr::X => self.var,
            Expr::Neg(a) => -self.evaluate(*a),
            Expr::Add(a, b) => {
                self.evaluate(*a) + self.evaluate(*b)
            },
            Expr::Sub(a, b) => {
                self.evaluate(*a) - self.evaluate(*b)
            },
            Expr::Mul(a, b) => {
                self.evaluate(*a) * self.evaluate(*b)
            },
            Expr::Div(a, b) => {
                self.evaluate(*a) / self.evaluate(*b)
            },
        }
    }
}
