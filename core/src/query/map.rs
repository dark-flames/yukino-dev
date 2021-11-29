use std::marker::PhantomData;

use query_builder::{OrderByItem, SelectQuery, SelectSource};

use crate::query::{AliasGenerator, ExecutableSelectQuery, ExecuteResultType};
use crate::view::{ValueCount, ViewBox};

pub struct QueryResultMap<R: 'static, RL: ValueCount, ResultType: ExecuteResultType> {
    query: Box<dyn SelectSource>,
    order_by_items: Vec<OrderByItem>,
    view: ViewBox<R, RL>,
    alias_generator: AliasGenerator,
    _result_ty: PhantomData<ResultType>,
}

pub trait Map<View> {
    type ResultType: ExecuteResultType;
    fn map<R: 'static, RL: ValueCount, RV: Into<ViewBox<R, RL>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RL, Self::ResultType>;
}

impl<R: 'static, RL: ValueCount, ResultType: ExecuteResultType> QueryResultMap<R, RL, ResultType> {
    pub fn create(
        query: Box<dyn SelectSource>,
        order_by_items: Vec<OrderByItem>,
        view: ViewBox<R, RL>,
        alias_generator: AliasGenerator,
    ) -> Self {
        QueryResultMap {
            query,
            order_by_items,
            view,
            alias_generator,
            _result_ty: Default::default(),
        }
    }
}

impl<R: 'static, RL: ValueCount, ResultType: ExecuteResultType> ExecutableSelectQuery<R, RL>
    for QueryResultMap<R, RL, ResultType>
{
    type ResultType = ResultType;

    fn generate_query(self) -> (SelectQuery, ViewBox<R, RL>) {
        let view = self.view.view_clone();
        (
            SelectQuery::create(
                self.query,
                self.alias_generator
                    .generate_select_list(self.view.collect_expr()),
                self.order_by_items,
                None,
                0,
            ),
            view,
        )
    }
}
