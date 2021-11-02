use crate::interface::def::FieldDefinition;
use crate::query::Alias;
use crate::view::{Value, View};

pub trait FieldMarker {
    type ValueType: Value;

    fn field_name() -> &'static str;

    fn definition() -> &'static FieldDefinition;
}

pub trait Entity: Value {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: View<Self::Entity> {
    type Entity: Entity;

    fn pure(alias: Alias) -> Self
    where
        Self: Sized;
}
