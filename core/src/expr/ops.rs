use crate::expr::value::Value;
use crate::expr::Expr;
use std::ops::Add;

pub trait ExprAdd<Rhs: Value = Self>: Value {
    type Output: Value;

    fn add(l: &Expr<Self>, r: &Expr<Rhs>) -> Expr<Self::Output>
        where
            Self: Sized;
}

impl<L, R, O> Add<&Expr<R>> for &Expr<L>
    where
        L: ExprAdd<R, Output=O>,
        R: Value,
        O: Value,
{
    type Output = Expr<O>;

    fn add(self, r: &Expr<R>) -> Self::Output {
        L::add(self, r)
    }
}
