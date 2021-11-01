use crate::converter::Converter;
use crate::expr::{ComputationNode, Expr};
use crate::interface::def::FieldDefinition;
use crate::view::View;

pub trait FieldMarker {
    type ValueType: 'static + Clone;

    fn field_name() -> &'static str;

    fn converter() -> &'static dyn Converter<Output=Self::ValueType>;

    fn definition() -> &'static FieldDefinition;

    fn view() -> &'static Expr<Self::ValueType>;
}

pub trait Entity: Clone {
    type View: EntityView<Entity = Self>;
}

pub trait EntityView: View<Output=Self::Entity> + ComputationNode + Clone {
    type Entity: Entity;
    fn pure() -> Self
        where
            Self: 'static + Sized,
    {
        Self::static_ref().clone()
    }

    fn static_ref() -> &'static Self
        where
            Self: 'static + Sized;
}
