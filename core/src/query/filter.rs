use generic_array::typenum::U1;

use query_builder::{Alias, ExprNode, SelectFrom};

use crate::query::{
    AliasGenerator, Fold, FoldedQueryResult, GroupBy, GroupedQueryResult, Map, QueryResultMap,
    SuitForGroupByList,
};
use crate::view::{
    AggregateView, EntityView, EntityWithView, ExprViewBox, Value, ValueCount, ViewBox,
};

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

        self.query.add_joins(visitor.joins());
        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });
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
        mut self,
        f: F,
    ) -> FoldedQueryResult<R, RV> {
        let mut entity_view = E::View::pure(&self.root_alias);
        let mut visitor = self.alias_generator.substitute_visitor();
        entity_view.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());
        FoldedQueryResult::create(Box::new(self.query), f(&entity_view))
    }
}

impl<E: EntityWithView> Map<E, E::View> for QueryResultFilter<E> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(&E::View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let entity_view = E::View::pure(&self.root_alias);
        let result_view = f(&entity_view).into();

        QueryResultMap::create(Box::new(self.query), result_view)
    }
}
