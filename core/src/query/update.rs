use std::collections::HashMap;
use std::marker::PhantomData;

use generic_array::ArrayLength;

use query_builder::{Expr, SelectFrom};

use crate::view::{EntityWithView, ExprViewBoxWithTag, FieldMarker, TagList, Value};

pub struct UpdateQuery<E: EntityWithView> {
    _source: SelectFrom,
    modify: HashMap<String, Expr>,
    _entity: PhantomData<E>
}

pub trait Update<E: EntityWithView> {
    fn update(self) -> UpdateQuery<E>;
}

impl<E: EntityWithView> UpdateQuery<E> {
    pub fn create(source: SelectFrom) -> Self {
        UpdateQuery {
            _source: source,
            modify: HashMap::new(),
            _entity: PhantomData
        }
    }
    pub fn assign<
        F: FieldMarker<Entity=E, FieldType=T>,
        T: Value,  Tags: TagList,
        V: Into<ExprViewBoxWithTag<T, Tags>>
    >(mut self, v: V) -> Self
        where <T as Value>::L: ArrayLength<(std::string::String, Expr)> {
        let view = v.into();
        let pairs = F::columns().into_iter().zip(view.collect_expr().into_iter());

        self.modify.extend(pairs);

        self
    }
}