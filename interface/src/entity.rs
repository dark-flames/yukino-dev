use crate::{EntityDefinition, FieldDefinition};

pub trait FieldMarker {
    type Entity: YukinoEntity;
    type FieldType;

    fn field_name() -> &'static str;

    fn definition() -> &'static FieldDefinition;
}

pub trait YukinoEntity: 'static {
    fn definition() -> &'static EntityDefinition;

    fn entity_id() -> usize;
}
