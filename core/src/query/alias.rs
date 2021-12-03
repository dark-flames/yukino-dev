use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::repeat;

use rand::{distributions::Alphanumeric, Rng, thread_rng};
use rand::rngs::ThreadRng;

use interface::DefinitionManager;
use query_builder::{Alias, Expr, SelectItem};

use crate::view::EntityWithView;

pub type AliasName = String;
pub type TableName = String;

pub struct AliasGenerator {
    rng: ThreadRng,
    _definition_manager: &'static DefinitionManager,
    alias: HashMap<AliasName, usize>,
    _path: HashMap<(String, String), AliasName>,
}

impl AliasGenerator {
    pub fn create(_definition_manager: &'static DefinitionManager) -> AliasGenerator {
        AliasGenerator {
            rng: thread_rng(),
            _definition_manager,
            alias: Default::default(),
            _path: Default::default(),
        }
    }

    pub fn generate_root_alias<E: EntityWithView>(&mut self) -> Alias {
        self.generate_alias(E::entity_id())
    }

    pub fn generate_select_list(&self, exprs: impl IntoIterator<Item = Expr>) -> Vec<SelectItem> {
        exprs
            .into_iter()
            .enumerate()
            .map(|(index, expr)| SelectItem {
                expr,
                alias: format!("result_{}", index),
            })
            .collect()
    }

    fn generate_alias(&mut self, entity_id: usize) -> Alias {
        let name = loop {
            let alias_name: String = repeat(())
                .map(|()| self.rng.sample(Alphanumeric))
                .map(char::from)
                .take(3)
                .collect();

            if let Entry::Vacant(e) = self.alias.entry(alias_name.clone()) {
                e.insert(entity_id);
                break alias_name;
            }
        };

        Alias { name }
    }
}
