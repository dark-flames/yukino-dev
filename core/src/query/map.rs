use query_builder::{SelectItem, SelectQuery, SelectSource};

use crate::query::AliasGenerator;
use crate::view::{ValueCount, ViewBox};

pub struct SingleRow;
pub struct MultiRows;

pub struct QueryResultMap<R: 'static, RL: ValueCount> {
    query: Box<dyn SelectSource>,
    view: ViewBox<R, RL>,
    alias_generator: AliasGenerator,
}

pub trait Map<View> {
    type ResultType;
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL>;
}

impl<R: 'static, RL: ValueCount> QueryResultMap<R, RL> {
    pub fn create(
        query: Box<dyn SelectSource>,
        view: ViewBox<R, RL>,
        alias_generator: AliasGenerator,
    ) -> Self {
        QueryResultMap {
            query,
            view,
            alias_generator,
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
