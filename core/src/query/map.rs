use query_builder::SelectSource;

use crate::view::{ExprView, Value, ValueCount, ViewBox};

#[allow(dead_code)]
pub struct QueryResultMap<R: 'static, RL: ValueCount> {
    query: Box<dyn SelectSource>,
    view: ViewBox<R, RL>,
}

pub trait Map<V: Value, View: ExprView<V>> {
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(&View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL>;
}

impl<R: 'static, RL: ValueCount> QueryResultMap<R, RL> {
    pub fn create(query: Box<dyn SelectSource>, view: ViewBox<R, RL>) -> Self {
        QueryResultMap { query, view }
    }
}
