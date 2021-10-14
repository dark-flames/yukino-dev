use crate::resolver::field::ResolvedField;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct UnassembledEntity {
    id: usize,
}

impl UnassembledEntity {
    pub fn assemble(self, fields: HashMap<String, ResolvedField>) -> ResolvedEntity {
        ResolvedEntity {
            id: self.id,
            fields,
        }
    }
}

pub struct ResolvedEntity {
    pub id: usize,
    pub fields: HashMap<String, ResolvedField>,
}

#[derive(Default)]
pub struct EntityResolver {
    unassembled: HashMap<usize, UnassembledEntity>,
    resolved: HashMap<usize, ResolvedEntity>,
}

impl EntityResolver {
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
}
