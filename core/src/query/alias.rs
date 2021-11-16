use crate::view::EntityWithView;
use interface::{
    AssociatedDefinition, DefinitionManager, FieldDefinition,
};
use query_builder::{Alias, AliasedTable, Expr, Ident, Join};
use rand::rngs::ThreadRng;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::repeat;
use std::marker::PhantomData;

pub type AliasName = String;
pub type TableName = String;

pub struct AliasGenerator<E: EntityWithView> {
    rng: ThreadRng,
    _root: Alias,
    definition_manager: &'static DefinitionManager,
    alias: HashMap<AliasName, usize>,
    path: HashMap<(String, String), AliasName>,
    _root_entity: PhantomData<E>,
}

impl<E: EntityWithView> AliasGenerator<E> {
    pub fn handle_ident(&mut self, ident: Ident) -> (Ident, Vec<Join>) {
        if ident.seg.len() == 1 {
            (ident, vec![])
        } else {
            let mut seg = ident.seg;
            let mut joins = vec![];

            while seg.len() > 1 {
                let prefix = (seg[0].clone(), seg[1].clone());

                let entry = self.path.entry(prefix.clone());

                match entry {
                    Entry::Occupied(e) => {
                        seg.pop();
                        seg[0] = e.get().clone();
                    }
                    Entry::Vacant(_) => {
                        let entity_id = self.alias.get(&prefix.0).unwrap();
                        let definition = self.definition_manager.entity(entity_id).unwrap();
                        let field_definition = definition.fields.get(&prefix.1).unwrap();

                        if let Some((join, alias)) =
                        self.handle_association(&prefix.0, field_definition)
                        {
                            joins.push(join);
                            seg.pop();
                            seg[0] = alias;
                        } else {
                            break;
                        }
                    }
                }
            }

            assert!(seg.len() <= 2);

            (Ident { seg }, joins)
        }
    }
    /**

     */

    fn generate_alias(&mut self, root: AliasName, field_name: String) -> Alias {
        let entry = self.path.entry((root, field_name));

        Alias {
            name: entry
                .or_insert_with(|| {
                    repeat(())
                        .map(|()| self.rng.sample(Alphanumeric))
                        .map(char::from)
                        .take(3)
                        .collect()
                })
                .clone(),
        }
    }

    fn handle_association(
        &mut self,
        alias: &str,
        field: &'static FieldDefinition,
    ) -> Option<(Join, AliasName)> {
        let l_alias = Alias {
            name: alias.to_string(),
        };
        match &field.association {
            Some(AssociatedDefinition::AssociatedEntity { ty, entity_id, map }) => {
                let r_alias = self.generate_alias(alias.to_string(), field.name.clone());
                let table = self
                    .definition_manager
                    .entity(entity_id)
                    .unwrap()
                    .name
                    .clone();

                let mut exprs: Vec<_> = map
                    .iter()
                    .map(|(l, r)| {
                        Expr::Eq(
                            Box::new(l_alias.create_ident_expr(l)),
                            Box::new(r_alias.create_ident_expr(r)),
                        )
                    })
                    .collect();

                let first = exprs.pop().unwrap();

                Some((
                    Join {
                        ty: *ty,
                        table: AliasedTable {
                            table,
                            alias: r_alias.clone(),
                        },
                        on: exprs
                            .into_iter()
                            .fold(first, |c, i| Expr::And(Box::new(c), Box::new(i))),
                    },
                    r_alias.name,
                ))
            }
            Some(AssociatedDefinition::ReversedAssociatedEntity {
                     ty,
                     entity_id,
                     field,
                 }) => {
                if let AssociatedDefinition::AssociatedEntity {
                    ty: _,
                    entity_id: _,
                    map,
                } = self
                    .definition_manager
                    .field(entity_id, field)
                    .unwrap()
                    .association
                    .as_ref()
                    .unwrap()
                {
                    let r_alias = self.generate_alias(alias.to_string(), field.clone());
                    let table = self
                        .definition_manager
                        .entity(entity_id)
                        .unwrap()
                        .name
                        .clone();

                    let mut exprs: Vec<_> = map
                        .iter()
                        .map(|(r, l)| {
                            Expr::Eq(
                                Box::new(l_alias.create_ident_expr(l)),
                                Box::new(r_alias.create_ident_expr(r)),
                            )
                        })
                        .collect();

                    let first = exprs.pop().unwrap();

                    Some((
                        Join {
                            ty: *ty,
                            table: AliasedTable {
                                table,
                                alias: r_alias.clone(),
                            },
                            on: exprs
                                .into_iter()
                                .fold(first, |c, i| Expr::And(Box::new(c), Box::new(i))),
                        },
                        r_alias.name,
                    ))
                } else {
                    unreachable!()
                }
            }
            None => None,
        }
    }
}
