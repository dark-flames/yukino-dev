use crate::entity::attr::IndexMethod;
use iroha::ToTokens;
use std::collections::HashMap;

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::entity::def")]
pub struct EntityDefinition {
    pub id: usize,
    pub name: String,
    pub definition_ty: DefinitionType,
    pub fields: HashMap<String, FieldDefinition>,
    pub indexes: Vec<IndexDefinition>,
    pub primary: Vec<String>,
    pub table_name: String,
}

#[derive(ToTokens)]
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

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::entity::def")]
pub enum IndexType {
    Primary,
    Unique,
    Normal,
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::entity::def")]
pub struct IndexDefinition {
    pub fields: Vec<String>,
    pub unique: IndexType,
    pub method: IndexMethod,
}

#[derive(ToTokens)]
#[Iroha(mod_path = "yukino::entity::def")]
pub enum DefinitionType {
    Normal,
    Visual,
}

#[derive(ToTokens)]
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
