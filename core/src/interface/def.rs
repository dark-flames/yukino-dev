use crate::interface::attr::IndexMethod;
use iroha::ToTokens;
use query_builder::DatabaseType;
use std::collections::HashMap;

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub struct EntityDefinition {
    pub id: usize,
    pub name: String,
    pub definition_ty: DefinitionType,
    pub fields: HashMap<String, FieldDefinition>,
    pub indexes: HashMap<String, IndexDefinition>,
    pub unique_primary: String,
    pub primary_fields: Vec<String>,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub struct FieldDefinition {
    pub name: String,
    pub ty: String,
    pub auto_increase: bool,
    pub definition_ty: DefinitionType,
    pub columns: HashMap<String, ColumnDefinition>,
    pub identity_columns: Vec<String>,
    pub association: Option<AssociatedDefinition>,
    pub indexes: Vec<IndexDefinition>,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub ty: IndexType,
    pub method: IndexMethod,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub struct ColumnDefinition {
    pub name: String,
    pub ty: DatabaseType,
    pub nullable: bool,
    pub auto_increase: bool,
}

#[derive(ToTokens, Copy, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub enum IndexType {
    Primary,
    Unique,
    Normal,
}

#[derive(ToTokens, Clone, Copy, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::interface::def")]
pub enum DefinitionType {
    Normal,
    Visual,
    Generated,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::interface::def")]
pub enum AssociatedDefinition {
    AssociatedEntity {
        entity_id: usize,
        map: HashMap<String, String>,
    },
    ReversedAssociatedEntity {
        entity_id: usize,
        field: String,
    },
}
