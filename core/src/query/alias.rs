use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::repeat;

use rand::{distributions::Alphanumeric, Rng, thread_rng};
use rand::rngs::ThreadRng;

use query_builder::{Alias, Expr, SelectItem};

use crate::view::EntityWithView;

pub type AliasName = String;
pub type TableName = &'static str;

pub struct AliasGenerator {
    rng: ThreadRng,
    alias: HashMap<AliasName, TableName>,
    _path: HashMap<(String, String), AliasName>,
}

impl AliasGenerator {
    pub fn create() -> AliasGenerator {
        AliasGenerator {
            rng: thread_rng(),
            alias: Default::default(),
            _path: Default::default(),
        }
    }

    pub fn generate_root_alias<E: EntityWithView>(&mut self) -> Alias {
        self.generate_alias(E::table_name())
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

    fn generate_alias(&mut self, table_name: &'static str) -> Alias {
        let name = loop {
            let alias_name: String = repeat(())
                .map(|()| self.rng.sample(Alphanumeric))
                .map(char::from)
                .take(3)
                .collect();

            if let Entry::Vacant(e) = self.alias.entry(alias_name.clone()) {
                e.insert(table_name);
                break alias_name;
            }
        };

        Alias { name }
    }
}
