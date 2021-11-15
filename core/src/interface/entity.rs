use crate::interface::def::FieldDefinition;
use crate::view::{ExprView, Value};
use query_builder::Alias;

pub trait FieldMarker {
    type Entity: Entity;

    fn field_name() -> &'static str;

    fn definition() -> &'static FieldDefinition;
}

pub trait Entity: Value {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: ExprView<Self::Entity> {
    type Entity: Entity;

    fn pure(alias: &Alias) -> Self
    where
        Self: Sized;
}
