use crate::expr::Expr;
use crate::query::calc::Computation;
use crate::query::optimizer::QueryOptimizer;
use std::any::type_name;

pub trait FieldMarker: Sized + 'static {
    type Type;

    fn type_name() -> String {
        type_name::<Self::Type>().to_string()
    }
}

pub trait Entity: Clone {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView {
    type Entity: Entity;
    fn pure() -> Self
    where
        Self: Sized;

    fn get<M: FieldMarker>(&self) -> Box<dyn FieldView<Type = M::Type>>
    where
        Self: Sized;
}

impl<E: Entity> Expr for dyn EntityView<Entity = E> {
    type Output = E;
    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        todo!()
    }

    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        todo!()
    }
}

pub trait FieldView {
    type Type;
}
