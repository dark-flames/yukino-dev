use std::marker::PhantomData;

pub trait FieldMarker {
    type Type;
}

pub trait Entity {
    type View: EntityView;
}

pub trait EntityView {
    type Entity: Entity;
    fn pure() -> Self;

    fn get<M: FieldMarker>() -> FieldView<M::Type>;
}

pub struct FieldView<T> {
    _marker: PhantomData<T>,
}
