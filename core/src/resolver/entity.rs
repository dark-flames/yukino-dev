use crate::resolver::field::ResolvedField;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use syn::ItemStruct;

#[allow(dead_code)]
pub struct UnassembledEntity {
    id: usize,
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
    #[allow(unreachable_code)]
    pub fn resolve(&mut self, _entity: &ItemStruct) -> (usize, usize) {
        todo!("Resolve entity");
        let unassembled_entity = UnassembledEntity { id: self.counter };

        self.unassembled.insert(self.counter, unassembled_entity);

        // (entity_id, field_count)
        (self.counter, 0)
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
