use std::hash::Hash;

#[allow(dead_code)]
pub struct FieldResolver {
    status: FieldResolverStatus,
}

#[derive(Debug, Clone, Hash)]
pub struct FieldPath {
    entity_id: usize,
    field_name: String,
}

#[allow(dead_code)]
pub enum FieldResolverStatus {
    WaitingForFields(Vec<FieldPath>),
    WaitingForEntity(usize),
    WaitingAssemble,
}
