use std::collections::hash_map::Entry;
use std::collections::HashMap;

use query_builder::{Alias, Expr, SelectItem};

use crate::view::EntityWithView;

pub type AliasName = String;
pub type TableName = &'static str;

#[derive(Clone)]
pub struct AliasGenerator {
    alias: HashMap<AliasName, TableName>
}

impl AliasGenerator {
    pub fn create() -> AliasGenerator {
        AliasGenerator {
            alias: Default::default()
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
        let mut offset = 1;
        let name = loop {
            let alias_name: String = format!("{}_{}", table_name, offset);

            if let Entry::Vacant(e) = self.alias.entry(alias_name.clone()) {
                e.insert(table_name);
                break alias_name;
            }
            offset += 1;
        };

        Alias { name }
    }
}
