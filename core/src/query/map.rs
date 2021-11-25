use query_builder::{SelectItem, SelectQuery, SelectSource};

use crate::query::AliasGenerator;
use crate::view::{ExprView, Value, ValueCount, ViewBox};

#[allow(dead_code)]
pub struct QueryResultMap<R: 'static, RL: ValueCount> {
    query: Box<dyn SelectSource>,
    alias_generator: AliasGenerator,
    view: ViewBox<R, RL>,
}

pub trait Map<V: Value, View: ExprView<V>> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL>;
}

impl<R: 'static, RL: ValueCount> QueryResultMap<R, RL> {
    pub fn create(
        query: Box<dyn SelectSource>,
        alias_generator: AliasGenerator,
        view: ViewBox<R, RL>,
    ) -> Self {
        QueryResultMap {
            query,
            alias_generator,
            view,
        }
    }

    pub fn generate_query(mut self) -> SelectQuery {
        let mut visitor = self.alias_generator.substitute_visitor();
        self.view.apply_mut(&mut visitor);
        SelectQuery::create(
            self.query,
            self.view
                .collect_expr()
                .into_iter()
                .enumerate()
                .map(|(i, e)| SelectItem {
                    expr: e,
                    alias: i.to_string(),
                })
                .collect(),
            vec![],
            None,
            0,
        )

        //todo: order limit offset
    }
}
