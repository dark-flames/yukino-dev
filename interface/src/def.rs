use crate::{DatabaseType, IndexMethod};
use iroha::ToTokens;
use std::collections::HashMap;

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino")]
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
#[Iroha(mod_path = "yukino")]
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
#[Iroha(mod_path = "yukino")]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub ty: IndexType,
    pub method: IndexMethod,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino")]
pub struct ColumnDefinition {
    pub name: String,
    pub ty: DatabaseType,
    pub nullable: bool,
    pub auto_increase: bool,
}

#[derive(ToTokens, Copy, Clone)]
#[Iroha(mod_path = "yukino")]
pub enum IndexType {
    Primary,
    Unique,
    Normal,
}

#[derive(ToTokens, Clone, Copy, Eq, PartialEq)]
#[Iroha(mod_path = "yukino")]
pub enum DefinitionType {
    Normal,
    Visual,
    Generated,
}

#[derive(Clone, Copy, Eq, PartialEq, ToTokens)]
#[Iroha(mod_path = "yukino")]
pub enum JoinType {
    LeftJoin,
    RightJoin,
    InnerJoin,
}

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino")]
pub enum AssociatedDefinition {
    AssociatedEntity {
        ty: JoinType,
        entity_id: usize,
        map: HashMap<String, String>,
    },
    ReversedAssociatedEntity {
        ty: JoinType,
        entity_id: usize,
        field: String,
    },
}

pub struct DefinitionManager {
    definitions: HashMap<usize, &'static EntityDefinition>,
}

impl DefinitionManager {
    pub fn create(items: Vec<(usize, &'static EntityDefinition)>) -> DefinitionManager {
        DefinitionManager {
            definitions: items.into_iter().collect(),
        }
    }

    pub fn entity(&self, id: &usize) -> Option<&'static EntityDefinition> {
        self.definitions.get(id).copied()
    }

    pub fn field(&self, id: &usize, field: &str) -> Option<&'static FieldDefinition> {
        self.definitions.get(id).and_then(|f| f.fields.get(field))
    }
}
