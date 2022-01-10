use std::collections::HashMap;
use std::marker::PhantomData;

use generic_array::ArrayLength;

use interface::FieldMarker;
use query_builder::{AssignmentValue, Expr, OrderByItem, Query, SelectFrom, UpdateQuery};

use crate::operator::SortResult;
use crate::query::{Executable, SingleRow, Sort};
use crate::view::{
    EntityView, EntityWithView, ExprViewBox, ExprViewBoxWithTag, FieldMarkerWithView, TagList,
    TagsOfValueView, Value,
};

pub struct UpdateQueryResult<E: EntityWithView> {
    query: UpdateQuery,
    assignments: HashMap<String, AssignmentValue>,
    _entity: PhantomData<E>,
}

pub trait Update<E: EntityWithView> {
    fn update(self) -> UpdateQueryResult<E>;
}

impl<E: EntityWithView> UpdateQueryResult<E> {
    pub fn create(source: SelectFrom) -> Self {
        UpdateQueryResult {
            query: source.into(),
            assignments: HashMap::new(),
            _entity: PhantomData,
        }
    }

    pub fn create_with_orders(source: SelectFrom, order_by_items: Vec<OrderByItem>) -> Self {
        let mut result = Self::create(source);
        result.query.append_order_by(order_by_items);

        result
    }

    #[must_use]
    pub fn set<
        FMarker: FieldMarkerWithView<Entity = E, FieldType = T>,
        T: Value,
        Tags: TagList,
        V: Into<ExprViewBoxWithTag<T, Tags>>,
    >(
        mut self,
        _m: FMarker,
        v: V,
    ) -> Self
    where
        <T as Value>::L: ArrayLength<(String, Expr)>,
    {
        let result = v.into();
        let pairs = FMarker::columns()
            .into_iter()
            .zip(result.collect_expr().into_iter().map(
                |e| AssignmentValue::Expr(Box::new(e))
            ));

        self.assignments.extend(pairs);

        self
    }

    #[must_use]
    pub fn set_by<
        FMarker: FieldMarkerWithView<Entity = E, FieldType = T>,
        T: Value,
        Tags: TagList,
        V: Into<ExprViewBoxWithTag<T, Tags>>,
        F: Fn(ExprViewBoxWithTag<T, FMarker::ViewTags>) -> V,
    >(
        mut self,
        _m: FMarker,
        f: F,
    ) -> Self {
        let result = f(FMarker::view(E::View::pure(self.query.root_alias()))).into();
        let pairs = FMarker::columns()
            .into_iter()
            .zip(result.collect_expr().into_iter().map(
                |e| AssignmentValue::Expr(Box::new(e))
            ));

        self.assignments.extend(pairs);

        self
    }

    #[must_use]
    pub fn set_default<FMarker: FieldMarkerWithView<Entity = E>>(mut self, _m: FMarker) -> Self
    where
        <FMarker as FieldMarker>::FieldType: Value,
    {
        self.assignments.extend(
            FMarker::columns()
                .into_iter()
                .map(|name| (name, AssignmentValue::Default)),
        );

        self
    }

    #[must_use]
    pub fn limit(mut self, l: usize) -> Self {
        self.query.limit(l);

        self
    }
}

impl<E: EntityWithView> Sort<E::View> for UpdateQueryResult<E> {
    type Result = UpdateQueryResult<E>;

    fn sort<R: SortResult, F: Fn(E::View) -> R>(mut self, f: F) -> Self::Result {
        let result = f(E::View::pure(self.query.root_alias()));

        self.query.append_order_by(result.order_by_items());

        self
    }
}

impl<E: EntityWithView> Executable<(), TagsOfValueView<()>> for UpdateQueryResult<E> {
    type ResultType = SingleRow;

    fn generate_query(mut self) -> (Query, ExprViewBox<()>) {
        for (column, value) in self.assignments {
            self.query.set(column, value);
        }

        (Query::Update(self.query), ().view())
    }
}
