use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::repeat;

use rand::{distributions::Alphanumeric, Rng, thread_rng};
use rand::rngs::ThreadRng;

use interface::{AssociatedDefinition, DefinitionManager, FieldDefinition, JoinType};
use query_builder::{Alias, AliasedTable, Expr, Ident, Join};

use crate::query::ExprMutVisitor;
use crate::view::EntityWithView;

pub type AliasName = String;
pub type TableName = String;

pub struct AliasGenerator {
    rng: ThreadRng,
    definition_manager: &'static DefinitionManager,
    alias: HashMap<AliasName, usize>,
    path: HashMap<(String, String), AliasName>,
}

impl AliasGenerator {
    pub fn create(definition_manager: &'static DefinitionManager) -> AliasGenerator {
        AliasGenerator {
            rng: thread_rng(),
            definition_manager,
            alias: Default::default(),
            path: Default::default(),
        }
    }

    pub fn substitute_visitor(&mut self) -> IdentSubstituteVisitor {
        IdentSubstituteVisitor::create(self)
    }

    pub fn generate_root_alias<E: EntityWithView>(&mut self) -> Alias {
        self.generate_alias(E::entity_id())
    }

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

    fn generate_alias_of_path(
        &mut self,
        root: AliasName,
        field_name: String,
        entity_id: usize,
    ) -> Alias {
        match self.path.entry((root.clone(), field_name.clone())) {
            Entry::Occupied(e) => Alias {
                name: e.get().clone(),
            },
            Entry::Vacant(_) => {
                let alias = self.generate_alias(entity_id);

                self.path.insert((root, field_name), alias.name.clone());

                alias
            }
        }
    }

    fn generate_join(table: String, alias: Alias, mut exprs: Vec<Expr>, ty: JoinType) -> Join {
        let first = exprs.pop().unwrap();
        Join {
            ty,
            table: AliasedTable { table, alias },
            on: exprs
                .into_iter()
                .fold(first, |c, i| Expr::And(Box::new(c), Box::new(i))),
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
                let r_alias =
                    self.generate_alias_of_path(alias.to_string(), field.name.clone(), *entity_id);
                let table = self
                    .definition_manager
                    .entity(entity_id)
                    .unwrap()
                    .name
                    .clone();

                let exprs = map
                    .iter()
                    .map(|(l, r)| {
                        Expr::Eq(
                            Box::new(l_alias.create_ident_expr(l)),
                            Box::new(r_alias.create_ident_expr(r)),
                        )
                    })
                    .collect();

                Some((
                    Self::generate_join(table, r_alias.clone(), exprs, *ty),
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
                    let r_alias =
                        self.generate_alias_of_path(alias.to_string(), field.clone(), *entity_id);
                    let table = self
                        .definition_manager
                        .entity(entity_id)
                        .unwrap()
                        .name
                        .clone();

                    let exprs: Vec<_> = map
                        .iter()
                        .map(|(r, l)| {
                            Expr::Eq(
                                Box::new(l_alias.create_ident_expr(l)),
                                Box::new(r_alias.create_ident_expr(r)),
                            )
                        })
                        .collect();

                    Some((
                        Self::generate_join(table, r_alias.clone(), exprs, *ty),
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

pub struct IdentSubstituteVisitor<'t> {
    generator: &'t mut AliasGenerator,
    joins: Vec<Join>,
}

impl<'t> IdentSubstituteVisitor<'t> {
    pub fn create(generator: &'t mut AliasGenerator) -> IdentSubstituteVisitor<'t> {
        IdentSubstituteVisitor {
            generator,
            joins: vec![],
        }
    }

    pub fn joins(self) -> Vec<Join> {
        self.joins
    }
}

impl<'t> ExprMutVisitor for IdentSubstituteVisitor<'t> {
    fn visit_ident(&mut self, node: &mut Ident) {
        let (ident, joins) = self.generator.handle_ident(node.clone());

        *node = ident;

        self.joins.extend(joins)
    }
}
