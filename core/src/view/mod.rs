mod basic;
use crate::expr::Expr;
pub use basic::*;

pub trait View {
    type Output: 'static + Clone;
    fn expr(&self) -> Expr<Self::Output>;
}

pub type ViewBox<V> = Box<dyn View<Output = V>>;
