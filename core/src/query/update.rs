use std::collections::HashMap;
use std::marker::PhantomData;

use generic_array::ArrayLength;

use query_builder::{AssignmentValue, Expr, OrderByItem, SelectFrom, UpdateQuery};

use crate::query::{Sort, SortHelper, SortResult};
use crate::view::{EntityView, EntityWithView, ExprViewBoxWithTag, FieldMarker, TagList, Value};

pub struct UpdateQueryResult<E: EntityWithView> {
    query: UpdateQuery,
    assign: HashMap<String, AssignmentValue>,
    _entity: PhantomData<E>
}

pub trait Update<E: EntityWithView> {
    fn update(self) -> UpdateQueryResult<E>;
}

impl<E: EntityWithView> UpdateQueryResult<E> {
    pub fn create(source: SelectFrom) -> Self {
        UpdateQueryResult {
            query: source.into(),
            assign: HashMap::new(),
            _entity: PhantomData
        }
    }

    pub fn create_with_orders(source: SelectFrom, order_by_items: Vec<OrderByItem>) -> Self {
        let mut result = Self::create(source);
        result.query.append_order_by(order_by_items);

        result
    }

    pub fn set<
        FMarker: FieldMarker<Entity=E, FieldType=T>,
        T: Value,  Tags: TagList,
        V: Into<ExprViewBoxWithTag<T, Tags>>
    >(mut self, v: V) -> Self where <T as Value>::L: ArrayLength<(String, Expr)> {
        let result = v.into();
        let pairs = FMarker::columns().into_iter().zip(result.collect_expr().into_iter()
            .map(AssignmentValue::Expr));

        self.assign.extend(pairs);

        self
    }

    pub fn set_by<
        FMarker: FieldMarker<Entity=E, FieldType=T>,
        T: Value,  Tags: TagList,
        V: Into<ExprViewBoxWithTag<T, Tags>>,
        F: Fn(ExprViewBoxWithTag<T, FMarker::ViewTags>) -> V
    >(mut self, f: F) -> Self {
        let result = f(FMarker::view(
            E::View::pure(self.query.root_alias())
        )).into();
        let pairs = FMarker::columns().into_iter().zip(result.collect_expr().into_iter()
            .map(AssignmentValue::Expr));

        self.assign.extend(pairs);

        self
    }

    pub fn set_default<
        FMarker: FieldMarker<Entity=E>,
    >(mut self) -> Self {
        self.assign.extend(FMarker::columns().into_iter().map(|name| (name, AssignmentValue::Default)));

        self
    }

    pub fn limit(mut self, l: usize) -> Self {
        self.query.limit(l);

        self
    }
}

impl<E: EntityWithView> Sort<E::View> for UpdateQueryResult<E> {
    type Result = UpdateQueryResult<E>;

    fn sort<R: SortResult, F: Fn(E::View, SortHelper) -> R>(mut self, f: F) -> Self::Result {
        let result = f(E::View::pure(self.query.root_alias()), SortHelper::create());

        self.query.append_order_by(result.order_by_items());

        self
    }
}