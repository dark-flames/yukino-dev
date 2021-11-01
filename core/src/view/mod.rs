mod basic;
use crate::expr::{Expr, Value};
pub use basic::*;

pub trait View {
    type Output: Value;
    fn expr(&self) -> Expr<Self::Output>;
}

pub type ViewBox<V> = Box<dyn View<Output = V>>;
