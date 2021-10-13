use crate::expr::Expr;
use crate::query::calc::Computation;
use crate::query::optimizer::QueryOptimizer;

pub trait FieldMarker {
    type Type;
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
