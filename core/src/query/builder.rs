use crate::query::Expr;
use crate::view::{Value, ViewBox};

#[allow(dead_code)]
pub struct QueryBuilder<T: Value> {
    filters: Vec<Expr>,
    view: ViewBox<T>,
}
