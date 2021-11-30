use std::marker::PhantomData;

use query_builder::{OrderByItem, SelectQuery, SelectSource};

use crate::query_result::{AliasGenerator, ExecutableSelectQuery, ExecuteResultType};
use crate::view::{ExprViewBoxWithTag, TagList, Value};

pub struct QueryResultMap<R: Value, RTags: TagList, ResultType: ExecuteResultType> {
    query: Box<dyn SelectSource>,
    order_by_items: Vec<OrderByItem>,
    view: ExprViewBoxWithTag<R, RTags>,
    alias_generator: AliasGenerator,
    _result_ty: PhantomData<ResultType>,
}

pub trait Map<View> {
    type ResultType: ExecuteResultType;
    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType>;
}

pub trait Map2<View1, View2> {
    type ResultType: ExecuteResultType;
    fn map<
        R: Value,
        RTags: TagList,
        RV: Into<ExprViewBoxWithTag<R, RTags>>,
        F: Fn(View1, View2) -> RV,
    >(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType>;
}

impl<R: Value, RTags: TagList, ResultType: ExecuteResultType> QueryResultMap<R, RTags, ResultType> {
    pub fn create(
        query: Box<dyn SelectSource>,
        order_by_items: Vec<OrderByItem>,
        view: ExprViewBoxWithTag<R, RTags>,
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

impl<R: Value, RTags: TagList, ResultType: ExecuteResultType> ExecutableSelectQuery<R, RTags>
    for QueryResultMap<R, RTags, ResultType>
{
    type ResultType = ResultType;

    fn generate_query(self) -> (SelectQuery, ExprViewBoxWithTag<R, RTags>) {
        let view = self.view.expr_clone();
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
