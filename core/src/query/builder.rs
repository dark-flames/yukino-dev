use crate::db::ty::DatabaseValue;
use crate::interface::{Entity, EntityView};
use crate::query::{Alias, TypedExpr};
use crate::view::{Value, ViewBox, ViewNode};
use std::marker::PhantomData;

pub struct QueryBuilder<E: Entity, T: Value> {
    filter: TypedExpr,
    view: ViewBox<T>,
    from_alias: Alias,
    _marker: PhantomData<E>,
}

impl<E: Entity, T: Value> QueryBuilder<E, T> {
    pub fn filter(mut self, f: impl Fn(E::View) -> ViewBox<bool>) -> QueryBuilder<E, T> {
        let view = E::View::pure(&self.from_alias);
        let result_view = f(view);
        match result_view.view_node() {
            ViewNode::Expr(expr) => {
                let left = self.filter.clone();
                self.filter = left.and(expr.exprs[0].clone()).unwrap()
                // TODO: Join
            }
            ViewNode::Const(expr) => {
                if expr.value {
                    self.filter = TypedExpr::lit(DatabaseValue::Bool(true)).unwrap()
                }
            }
            ViewNode::Computation(_) => {
                panic!("filter cannot handle computation view");
            }
        }

        self
    }

    pub fn map<V: Value>(
        self,
        f: impl Fn(E::View, ViewBox<T>) -> ViewBox<V>,
    ) -> QueryBuilder<E, V> {
        let entity_view = E::View::pure(&self.from_alias);
        let view = f(entity_view, self.view);

        QueryBuilder {
            filter: self.filter,
            view,
            from_alias: self.from_alias,
            _marker: Default::default(),
        }
    }

    pub fn execute(self) -> T {
        todo!()
    }
}
