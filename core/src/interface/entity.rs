use crate::interface::converter::DataConverter;
use crate::interface::def::FieldDefinition;
use crate::view::View;
use std::any::type_name;

pub trait FieldMarker {
    type Type;

    fn field_name() -> String;

    fn type_name() -> String {
        type_name::<Self::Type>().to_string()
    }

    fn data_converter() -> Box<dyn DataConverter<FieldType = Self::Type>>;

    fn definition() -> FieldDefinition;
}

pub trait Entity: Clone {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: View<Output = Self::Entity> {
    type Entity: Entity;
    fn pure() -> Self
    where
        Self: Sized;
}

pub trait FieldView: View<Output = Self::Type> {
    type Type;
}
