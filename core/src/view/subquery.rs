use generic_array::typenum::U1;

use query_builder::SelectQuery;

use crate::view::{ExprView, Value};

// Single column query can be a subquery
pub trait SubqueryView<T: Value<L=U1>> {
    fn subquery(&self) -> SelectQuery;
}

// Single row subquery can be a SingleRowSubquery, it can be use as a ExprView
pub trait SingleRowSubqueryView<T: Value<L=U1>>: SubqueryView<T> + ExprView<T> {
}