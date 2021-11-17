use crate::query::AliasGenerator;
use crate::view::{EntityWithView, ExprViewBox, Value};
use query_builder::GroupSelect;

pub trait GroupBy<E: EntityWithView> {
    fn group_by<V0: Value, R: Into<ExprViewBox<V0>>, F: Fn(E::View) -> R>(
        self,
        f: F,
    ) -> GroupedQueryResult<E, V0>;

    fn group_by2<V0: Value, V1: Value, R: Into<ExprViewBox<(V0, V1)>>, F: Fn(E::View) -> R>(
        self,
        f: F,
    ) -> GroupedQueryResult2<E, V0, V1>
        where
            (V0, V1): Value;
}

#[allow(dead_code)]
pub struct GroupedQueryResult<E: EntityWithView, V0: Value> {
    alias_generator: AliasGenerator,
    query: GroupSelect<E>,
    grouped_view: ExprViewBox<V0>,
}

#[allow(dead_code)]
pub struct GroupedQueryResult2<E: EntityWithView, V0: Value, V1: Value>
    where
        (V0, V1): Value,
{
    alias_generator: AliasGenerator,
    query: GroupSelect<E>,
    grouped_view: ExprViewBox<(V0, V1)>,
}
