use crate::interface::{Entity, EntityView};
use crate::query::{Alias, Expr};
use crate::view::{ExprView, Value, ViewBox};
use std::marker::PhantomData;

pub struct QueryBuilder<E: Entity, T: Value> {
    filter: Box<Expr>,
    view: ViewBox<T>,
    from_alias: Alias,
    _marker: PhantomData<E>,
}

impl<E: Entity, T: Value> QueryBuilder<E, T> {
    pub fn filter(mut self, f: impl Fn(E::View) -> Box<ExprView<bool>>) -> QueryBuilder<E, T> {
        let view = E::View::pure(&self.from_alias);
        let result_view = *f(view);
        let left = self.filter.clone();
        self.filter = Box::new(Expr::And(left, Box::new(result_view.exprs[0].clone())));

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
