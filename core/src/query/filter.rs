use interface::DefinitionManager;
use query_builder::{Alias, SelectFrom};

use crate::query::{AliasGenerator, Fold, FoldQueryResult, FoldQueryResult2, Map, QueryResultMap};
use crate::view::{
    AggregateViewTag, EntityView, EntityWithView, ExprViewBox, ExprViewBoxWithTag, InList, TagList,
    Value, ValueCount, ViewBox,
};

pub struct QueryResultFilter<E: EntityWithView> {
    query: SelectFrom<E>,
    root_alias: Alias,
    alias_generator: AliasGenerator,
}

pub trait Filter<View> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(self, f: F) -> Self
    where
        F: Fn(View) -> R;
}

impl<E: EntityWithView> Filter<E::View> for QueryResultFilter<E> {
    fn filter<F, R: Into<ExprViewBox<bool>>>(mut self, f: F) -> Self
    where
        F: Fn(E::View) -> R,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut view = f(entity_view).into();
        let mut visitor = self.alias_generator.substitute_visitor();
        view.apply_mut(&mut visitor);

        self.query.add_joins(visitor.joins());
        view.collect_expr().into_iter().for_each(|e| {
            self.query.and_where(e);
        });

        self
    }
}

impl<E: EntityWithView> Map<E::View> for QueryResultFilter<E> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(E::View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL> {
        let entity_view = E::View::pure(&self.root_alias);
        let result_view = f(entity_view).into();

        QueryResultMap::create(Box::new(self.query), self.alias_generator, result_view)
    }
}

impl<E: EntityWithView> QueryResultFilter<E> {
    pub fn create(manager: &'static DefinitionManager) -> Self {
        let mut generator = AliasGenerator::create(manager);
        let root_alias = generator.generate_root_alias::<E>();
        QueryResultFilter {
            query: SelectFrom::create(root_alias.clone()),
            root_alias,
            alias_generator: generator,
        }
    }
}

impl<E: EntityWithView> Fold<E::View> for QueryResultFilter<E> {
    fn fold<R1: Value, R1Tag: TagList, F: Fn(E::View) -> ExprViewBoxWithTag<R1, R1Tag>>(
        mut self,
        f: F,
    ) -> FoldQueryResult<R1, R1Tag>
    where
        AggregateViewTag: InList<R1Tag>,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut visitor = self.alias_generator.substitute_visitor();
        let mut result = f(entity_view);
        result.apply_mut(&mut visitor);

        FoldQueryResult::create(Box::new(self.query), self.alias_generator, result)
    }

    fn fold2<
        T1: Value,
        T1Tag: TagList,
        T2: Value,
        T2Tag: TagList,
        F: Fn(E::View) -> (ExprViewBoxWithTag<T1, T1Tag>, ExprViewBoxWithTag<T2, T2Tag>),
    >(
        mut self,
        f: F,
    ) -> FoldQueryResult2<T1, T1Tag, T2, T2Tag>
    where
        AggregateViewTag: InList<T1Tag> + InList<T2Tag>,
    {
        let entity_view = E::View::pure(&self.root_alias);
        let mut visitor = self.alias_generator.substitute_visitor();
        let mut result = f(entity_view);
        result.0.apply_mut(&mut visitor);
        result.1.apply_mut(&mut visitor);

        FoldQueryResult2::create(Box::new(self.query), self.alias_generator, result)
    }
}
