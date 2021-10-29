use crate::interface::converter::DataConverter;
use crate::interface::def::FieldDefinition;
use crate::view::View;

pub trait FieldMarker {
    type ValueType;

    fn field_name() -> &'static str;

    fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType>;

    fn definition() -> &'static FieldDefinition;
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

pub trait FieldView: View {
    type ConverterType: 'static + Clone;

    fn create(converter: &'static dyn DataConverter<Output=Self::ConverterType>) -> Self
        where
            Self: Sized;

    fn get_converter(&self) -> &'static dyn DataConverter<Output=Self::ConverterType>;
}
