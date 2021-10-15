use crate::entity::attr::IndexMethod;
use iroha::ToTokens;
use std::collections::HashMap;

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::entity::def")]
pub struct EntityDefinition {
    pub id: usize,
    pub name: String,
    pub definition_ty: DefinitionType,
    pub fields: HashMap<String, FieldDefinition>,
    pub indexes: HashMap<String, IndexDefinition>,
    pub primary: Vec<String>,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::entity::def")]
pub struct FieldDefinition {
    pub name: String,
    pub ty: usize,
    pub auto_increase: bool,
    pub definition_ty: DefinitionType,
    pub columns: Vec<String>,
    pub identity_columns: Vec<String>,
    pub association: Option<AssociatedDefinition>,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::entity::def")]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub ty: IndexType,
    pub method: IndexMethod,
}

#[derive(ToTokens, Copy, Clone)]
#[Iroha(mod_path = "yukino::entity::def")]
pub enum IndexType {
    Primary,
    Unique,
    Normal,
}

#[derive(ToTokens, Clone, Copy)]
#[Iroha(mod_path = "yukino::entity::def")]
pub enum DefinitionType {
    Normal,
    Visual,
    Generated,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::entity::def")]
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
