use crate::{EntityDefinition, FieldDefinition};

pub trait FieldMarker {
    type Entity: YukinoEntity;

    fn field_name() -> &'static str;

    fn definition() -> &'static FieldDefinition;
}

pub trait YukinoEntity: 'static {
    fn definition() -> &'static EntityDefinition;
}
