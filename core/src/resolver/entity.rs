use crate::db::ty::DatabaseType;
use crate::entity::attr::{Entity, Index, IndexMethod};
use crate::entity::def::{
    ColumnDefinition, DefinitionType, EntityDefinition, FieldDefinition, IndexDefinition, IndexType,
};
use crate::err::{ResolveError, YukinoError};
use crate::resolver::field::ResolvedField;
use crate::resolver::CliResult;
use annotation_rs::AnnotationStructure;
use heck::SnakeCase;
use proc_macro2::{Span, TokenStream};
use std::collections::HashMap;
use std::iter::Extend;
use syn::spanned::Spanned;
use syn::ItemStruct;

#[allow(dead_code)]
pub struct UnassembledEntity {
    id: usize,
    name: String,
    indexes: HashMap<String, Index>,
    span: Span,
}

pub struct ResolvedEntity {
    pub id: usize,
    pub definitions: Vec<EntityDefinition>,
    pub fields: HashMap<String, ResolvedField>,
}

#[derive(Default)]
pub struct EntityResolver {
    counter: usize,
    unassembled: HashMap<usize, UnassembledEntity>,
    resolved: HashMap<usize, ResolvedEntity>,
    passes: Vec<Box<dyn EntityResolvePass>>,
}

pub trait EntityResolvePass {
    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream>;

    fn get_additional_implements(&self) -> Vec<TokenStream>;
}

impl UnassembledEntity {
    pub fn assemble(self, fields: HashMap<String, ResolvedField>) -> CliResult<ResolvedEntity> {
        let mut field_definitions: HashMap<_, _> = fields
            .iter()
            .map(|(name, f)| (name.clone(), f.definition.clone()))
            .collect();
        // Indexes defined by user
        let mut indexes: Vec<_> = self
            .indexes
            .iter()
            .map(|(name, index)| {
                if let Some(index_field) = index.fields.iter().find(|&f| fields.contains_key(f)) {
                    Err(
                        ResolveError::IndexedFieldNotFound(index_field.clone(), name.clone())
                            .as_cli_err(Some(self.span.clone())),
                    )
                } else {
                    Ok((
                        name.clone(),
                        IndexDefinition {
                            name: name.clone(),
                            fields: index.fields.clone(),
                            ty: if index.unique {
                                IndexType::Unique
                            } else {
                                IndexType::Normal
                            },
                            method: index.method,
                        },
                    ))
                }
            })
            .collect::<CliResult<Vec<_>>>()?;

        let field_with_primary: Vec<_> = fields
            .iter()
            .filter_map(|(name, f)| f.primary.then(|| name.clone()))
            .collect();

        let unique_primary = if field_with_primary.len() == 1 {
            field_with_primary.first().unwrap().clone()
        } else {
            let generated_name = format!("_{}_id", &self.name);
            field_definitions.insert(
                generated_name.clone(),
                FieldDefinition {
                    name: generated_name.clone(),
                    ty: "String".to_string(),
                    auto_increase: false,
                    definition_ty: DefinitionType::Generated,
                    columns: vec![(
                        generated_name.clone(),
                        ColumnDefinition {
                            name: generated_name.clone(),
                            ty: DatabaseType::String,
                            nullable: false,
                            auto_increase: false,
                        },
                    )]
                    .into_iter()
                    .collect(),
                    identity_columns: vec![generated_name.clone()],
                    association: Option::None,
                    indexes: vec![],
                },
            );

            if !field_with_primary.is_empty() {
                let index_name = format!("_{}_primary", &self.name);
                indexes.push((
                    index_name.clone(),
                    IndexDefinition {
                        name: index_name,
                        fields: field_with_primary.clone(),
                        ty: IndexType::Primary,
                        method: IndexMethod::BTree,
                    },
                ));
            };

            generated_name
        };

        let mut definitions: Vec<_> = fields
            .values()
            .flat_map(|f| f.entities.clone().into_iter())
            .collect();

        definitions.push(EntityDefinition {
            id: self.id,
            name: self.name.clone(),
            definition_ty: DefinitionType::Normal,
            fields: field_definitions,
            indexes: indexes.into_iter().collect(),
            unique_primary,
            primary_fields: field_with_primary,
        });

        Ok(ResolvedEntity {
            id: self.id,
            definitions,
            fields,
        })
    }
}

impl EntityResolver {
    pub fn create(passes: Vec<Box<dyn EntityResolvePass>>) -> Self {
        EntityResolver {
            counter: 0,
            unassembled: Default::default(),
            resolved: Default::default(),
            passes,
        }
    }

    pub fn resolve(&mut self, entity: &ItemStruct) -> CliResult<(usize, usize)> {
        let entity_id = self.counter;
        self.counter += 1;
        let entity_name = entity.ident.to_string();

        let attribute: Entity = entity
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path == Entity::get_path() {
                    Some(
                        attr.parse_meta()
                            .map_err(|e| {
                                ResolveError::EntityParseError(entity_name.clone(), e.to_string())
                                    .as_cli_err(Some(e.span()))
                            })
                            .and_then(|meta| {
                                Entity::from_meta(&meta).map_err(|e| {
                                    ResolveError::EntityParseError(
                                        entity_name.clone(),
                                        e.to_string(),
                                    )
                                    .as_cli_err(Some(e.span()))
                                })
                            }),
                    )
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| {
                ResolveError::NoEntityAttribute(entity_name.clone()).as_cli_err(Some(entity.span()))
            })??;

        let unassembled_entity = UnassembledEntity {
            id: entity_id,
            name: attribute
                .name
                .clone()
                .unwrap_or_else(|| entity_name.to_snake_case()),
            indexes: attribute.indexes.unwrap_or_default(),
            span: entity.span(),
        };
        self.unassembled.insert(entity_id, unassembled_entity);

        Ok((entity_id, 0))
    }

    pub fn assembled(
        &mut self,
        entity_id: usize,
        fields: HashMap<String, ResolvedField>,
    ) -> CliResult<&ResolvedEntity> {
        let resolved = self
            .unassembled
            .remove(&entity_id)
            .unwrap()
            .assemble(fields)?;

        self.resolved.insert(entity_id, resolved);

        Ok(self.resolved.get(&entity_id).unwrap())
    }

    pub fn all_finished(&self) -> bool {
        self.unassembled.is_empty()
    }

    pub fn get_implements(&self) -> Vec<TokenStream> {
        assert!(self.all_finished());

        let mut implements: Vec<_> = self
            .resolved
            .values()
            .flat_map(|entity| {
                self.passes
                    .iter()
                    .flat_map(|pass| pass.get_entity_implements(entity))
            })
            .collect();

        implements.extend(
            self.passes
                .iter()
                .flat_map(|pass| pass.get_additional_implements()),
        );

        implements
    }

    pub fn get_definitions(&self) -> Vec<EntityDefinition> {
        assert!(self.all_finished());

        self.resolved
            .values()
            .flat_map(|entity| entity.definitions.clone().into_iter())
            .collect()
    }
}
