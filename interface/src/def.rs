use crate::DatabaseType;

#[derive(Debug)]
pub struct EntityDefinition {
    pub table_name: String,
    pub associations: Vec<AssociationDefinition>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug)]
pub struct AssociationDefinition {
    pub referenced_entity_name: String,
    pub field_name: String,
}

#[derive(Debug)]
pub struct FieldDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
    pub identity_column: String,
    pub primary_key: bool,
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub ty: DatabaseType,
    pub optional: bool,
    pub auto_increment: bool
}
