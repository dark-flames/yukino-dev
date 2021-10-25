use crate::interface::converter::DataConverter;
use crate::expr::View;
use std::any::type_name;

pub trait FieldMarker: Sized + 'static {
    type Type;

    fn type_name() -> String {
        type_name::<Self::Type>().to_string()
    }

    fn data_converter() -> Box<dyn DataConverter<FieldType=Self::Type>>;
}

pub trait Entity: Clone {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: View<Output=Self::Entity> {
    type Entity: Entity;
    fn pure() -> Self
        where
            Self: Sized;

    fn get<M: FieldMarker>(&self) -> Box<dyn FieldView<Type=M::Type, Output=M::Type>>
        where
            Self: Sized;
}

pub trait FieldView: View<Output=Self::Type> {
    type Type;
}
