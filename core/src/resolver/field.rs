use crate::err::{ResolveError, YukinoError};
use crate::resolver::entity::ResolvedEntity;
use crate::resolver::entry::CliResult;
use crate::resolver::path::FileTypePathResolver;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use syn::spanned::Spanned;
use syn::Field;

pub type EntityHandler = Box<dyn FnOnce(&ResolvedEntity) -> FieldResolveResult>;
pub type FieldHandler = Box<dyn FnOnce(&ResolvedField) -> FieldResolveResult>;
pub type ReadyEntities = HashSet<usize>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldPath {
    entity_id: usize,
    field_name: String,
}

#[derive(Clone)]
pub struct ResolvedField {
    pub path: FieldPath,
}

#[derive(Default)]
pub struct FieldResolver {
    cells: Vec<Box<dyn FieldResolverCell>>,
    waiting_for_field: HashMap<FieldPath, Vec<FieldHandler>>,
    waiting_for_entity: HashMap<usize, Vec<EntityHandler>>,
    finished: HashMap<usize, HashMap<String, ResolvedField>>,
    field_count: HashMap<usize, usize>,
}

pub enum FieldResolveResult {
    Finished(ResolvedField),
    WaitingForEntity(usize, EntityHandler),
    WaitingForField(FieldPath, FieldHandler),
}

pub trait FieldResolverCell {
    fn match_field(&self, field: &Field, type_resolver: &FileTypePathResolver) -> bool;

    fn resolve(
        &self,
        type_resolver: &FileTypePathResolver,
        field_path: FieldPath,
        field: &Field,
    ) -> CliResult<FieldResolveResult>;
}

impl FieldPath {
    pub fn create(entity_id: usize, field_name: String) -> Self {
        FieldPath {
            entity_id,
            field_name,
        }
    }
}

impl FieldResolver {
    pub fn create(cells: Vec<Box<dyn FieldResolverCell>>) -> Self {
        FieldResolver {
            cells,
            waiting_for_field: Default::default(),
            waiting_for_entity: Default::default(),
            finished: Default::default(),
            field_count: Default::default(),
        }
    }

    pub fn resolve(
        &mut self,
        type_resolver: &FileTypePathResolver,
        field_path: FieldPath,
        field: &Field,
    ) -> CliResult<ReadyEntities> {
        let resolver = self
            .cells
            .iter()
            .find(|c| c.match_field(field, type_resolver))
            .ok_or_else(|| {
                ResolveError::NoSuitableResolveCell(field_path.field_name.clone())
                    .as_cli_err(Some(field.span()))
            })?;

        let result = resolver.resolve(type_resolver, field_path, field)?;

        Ok(self.handle_resolve_result(result))
    }

    pub fn apply_entity(&mut self, resolved_entity: &ResolvedEntity) -> ReadyEntities {
        self.waiting_for_entity
            .remove(&resolved_entity.id)
            .unwrap_or_default()
            .into_iter()
            .flat_map(|handler| self.handle_resolve_result(handler(resolved_entity)))
            .collect()
    }

    pub fn get_entity_fields(&self, entity_id: usize) -> HashMap<String, ResolvedField> {
        assert!(self.finish_resolved_entity_fields(entity_id));

        self.finished[&entity_id].clone()
    }

    pub fn set_entity_field_count(&mut self, entity_id: usize, count: usize) {
        self.field_count.insert(entity_id, count);
    }

    pub fn all_finished(&self) -> bool {
        self.waiting_for_field.is_empty() && self.waiting_for_entity.is_empty()
    }

    fn finish_resolved_entity_fields(&self, entity_id: usize) -> bool {
        self.finished[&entity_id].len() == self.field_count[&entity_id]
    }

    fn handle_resolve_result(&mut self, result: FieldResolveResult) -> ReadyEntities {
        match result {
            FieldResolveResult::Finished(resolved) => {
                let mut ready_entities: ReadyEntities = self
                    .waiting_for_field
                    .remove(&resolved.path)
                    .unwrap_or_default()
                    .into_iter()
                    .flat_map(|handler| self.handle_resolve_result(handler(&resolved)))
                    .collect();

                let entity_id = resolved.path.entity_id;
                self.finished
                    .entry(entity_id)
                    .or_default()
                    .insert(resolved.path.field_name.clone(), resolved);

                if self.finish_resolved_entity_fields(entity_id) {
                    ready_entities.insert(entity_id);
                }

                ready_entities
            }
            FieldResolveResult::WaitingForEntity(entity_id, handler) => {
                self.waiting_for_entity
                    .entry(entity_id)
                    .or_default()
                    .push(handler);
                HashSet::default()
            }
            FieldResolveResult::WaitingForField(path, handler) => {
                self.waiting_for_field
                    .entry(path)
                    .or_default()
                    .push(handler);
                HashSet::default()
            }
        }
    }
}
