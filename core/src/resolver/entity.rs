use crate::entity::attr::{Entity, Index};
use crate::resolver::field::ResolvedField;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use syn::ItemStruct;
use crate::resolver::entry::CliResult;
use annotation_rs::AnnotationStructure;
use heck::SnakeCase;
use syn::spanned::Spanned;
use crate::err::{ResolveError, YukinoError};

#[allow(dead_code)]
pub struct UnassembledEntity {
    id: usize,
    name: String,
    indexes: HashMap<String, Index>
}

pub struct ResolvedEntity {
    pub id: usize,
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
    pub fn assemble(self, fields: HashMap<String, ResolvedField>) -> ResolvedEntity {
        ResolvedEntity {
            id: self.id,
            fields,
        }
    }
}

impl EntityResolver {
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
                            .map_err(
                                |e| ResolveError::EntityParseError(entity_name.clone(), e.to_string())
                                    .as_cli_err(Some(e.span()))
                            )
                            .and_then(|meta| Entity::from_meta(&meta).map_err(
                                |e| ResolveError::EntityParseError(entity_name.clone(), e.to_string())
                                    .as_cli_err(Some(e.span()))
                            ))
                    )
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(
                || ResolveError::NoEntityAttribute(entity_name.clone()).as_cli_err(Some(entity.span()))
            )??;

        let unassembled_entity = UnassembledEntity {
            id: entity_id,
            name: attribute.name.clone().unwrap_or_else(|| entity_name.to_snake_case()),
            indexes: attribute.indexes.unwrap_or_default()
        };
        self.unassembled.insert(entity_id, unassembled_entity);

        Ok((entity_id, 0))
    }

    pub fn assembled(
        &mut self,
        entity_id: usize,
        fields: HashMap<String, ResolvedField>,
    ) -> &ResolvedEntity {
        let resolved = self
            .unassembled
            .remove(&entity_id)
            .unwrap()
            .assemble(fields);

        self.resolved.insert(entity_id, resolved);

        self.resolved.get(&entity_id).unwrap()
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
}
