use crate::err::CliResult;
use crate::err::{ResolveError, YukinoError};
use crate::resolver::entity::ResolvedEntity;
use crate::resolver::path::FileTypePathResolver;
use interface::{EntityDefinition, FieldDefinition};
use proc_macro2::{Ident, TokenStream};
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use syn::spanned::Spanned;
use syn::Field;

pub type EntityHandler = Box<dyn FnOnce(&ResolvedEntity) -> FieldResolveResult>;
pub type FieldHandler = Box<dyn FnOnce(&ResolvedField) -> FieldResolveResult>;
pub type ReadyEntities = HashSet<usize>;
pub type FieldResolverSeedBox = Box<dyn FieldResolverSeed>;
pub type FieldResolverCellBox = Box<dyn FieldResolverCell>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldPath {
    pub entity_id: usize,
    pub field_name: String,
}

#[derive(Clone)]
pub struct ResolvedField {
    pub path: FieldPath,
    pub definition: FieldDefinition,
    pub converter: TokenStream,
    pub converter_type: TokenStream,
    pub converter_param_count: usize,
    pub ty: TokenStream,
    pub view: TokenStream,
    pub view_ty: TokenStream,
    pub view_path: TokenStream,
    pub marker_name: Ident,
    pub primary: bool,
    pub entities: Vec<EntityDefinition>,
}

#[derive(Default)]
pub struct FieldResolver {
    seeds: Vec<FieldResolverSeedBox>,
    waiting_for_field: HashMap<FieldPath, Vec<FieldHandler>>,
    waiting_for_entity: HashMap<usize, Vec<EntityHandler>>,
    finished: HashMap<usize, HashMap<String, ResolvedField>>,
    field_count: HashMap<usize, usize>,
}

pub enum FieldResolveResult {
    Finished(Box<ResolvedField>),
    WaitingForEntity(usize, EntityHandler),
    WaitingForField(FieldPath, FieldHandler),
}

pub trait FieldResolverSeed {
    fn instance() -> FieldResolverSeedBox
    where
        Self: Sized;
    fn match_field(
        &self,
        field: &Field,
        type_resolver: &FileTypePathResolver,
    ) -> CliResult<Option<FieldResolverCellBox>> where;
}

pub trait FieldResolverCell {
    fn wrap(self) -> FieldResolverCellBox
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }

    fn resolve(
        &self,
        type_resolver: &FileTypePathResolver,
        field_path: FieldPath,
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
    pub fn create(seeds: Vec<FieldResolverSeedBox>) -> Self {
        FieldResolver {
            seeds,
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
        let resolver_result = self
            .seeds
            .iter()
            .map(|s| s.match_field(field, type_resolver))
            .collect::<CliResult<Vec<_>>>()?;

        let resolver = resolver_result
            .iter()
            .filter_map(|r| r.as_ref())
            .next()
            .ok_or_else(|| {
                ResolveError::NoSuitableResolveSeed(field_path.field_name.clone())
                    .as_cli_err(Some(field.span()))
            })?;

        let result = resolver.resolve(type_resolver, field_path)?;

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
                    .insert(resolved.path.field_name.clone(), *resolved);

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
