use generic_array::typenum::U1;

use query_builder::{Alias, SelectFrom};

use crate::query::{
    AliasGenerator, Fold, FoldedQueryResult, GroupBy, GroupedQueryResult, SuitForGroupByList,
};
use crate::view::{AggregateView, EntityView, EntityWithView, ExprViewBox, Value};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    alias_generator: AliasGenerator,
}

pub trait Filter {
    type View;

    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
    where
        F: Fn(&Self::View) -> R;
}

impl<E: EntityWithView> Filter for QueryResultFilter<E> {
    type View = E::View;
    fn filter<F, R: Into<ExprViewBox<bool>>>(&mut self, f: F)
    where
        F: Fn(&Self::View) -> R,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut view = f(&entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);
        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });

        self.query.add_joins(visitor.joins());
    }
}

impl<E: EntityWithView> GroupBy<E> for QueryResultFilter<E> {
    fn group_by<Fields: SuitForGroupByList<Entity=E>>(self) -> GroupedQueryResult<E, Fields> {
        let QueryResultFilter {
            query,
            alias_generator,
            root_alias,
        } = self;

        GroupedQueryResult::create(
            query.group_by(
                Fields::idents(root_alias.single_seg_ident())
                    .into_iter()
                    .collect(),
            ),
            alias_generator,
            root_alias,
        )
    }
}

impl<E: EntityWithView> Fold<E, E::View> for QueryResultFilter<E> {
    fn fold<R: Value<L=U1>, RV: AggregateView<R>, F: Fn(&E::View) -> RV>(
        self,
        f: F,
    ) -> FoldedQueryResult<R, RV> {
        let entity_view = E::View::pure(&self.root_alias);
        FoldedQueryResult::create(Box::new(self.query), f(&entity_view))
    }
}
