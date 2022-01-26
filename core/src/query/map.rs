use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;
use sqlx::Database;

use query_builder::{Expr, OrderByItem, SelectQuery, SelectSource, YukinoQuery};

use crate::query::{AliasGenerator, Executable, ExecuteResultType, MultiRows, SingleRow};
use crate::view::{
    ExprView, ExprViewBox, ExprViewBoxWithTag, SingleRowSubqueryView, SubqueryIntoView,
    SubqueryView, TagList, Value, ValueCountOf,
};

#[derive(Clone)]
pub struct MappedQueryBuilder<R: Value, RTags: TagList, ResultType: ExecuteResultType> {
    query: SelectSource,
    order_by_items: Vec<OrderByItem>,
    view: ExprViewBoxWithTag<R, RTags>,
    alias_generator: AliasGenerator,
    limit: Option<usize>,
    offset: usize,
    _result_ty: PhantomData<ResultType>,
}

pub trait Map<View> {
    type ResultType: ExecuteResultType;
    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType>;
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
    ) -> MappedQueryBuilder<R, RTags, Self::ResultType>;
}

impl<R: Value, RTags: TagList, ResultType: ExecuteResultType> MappedQueryBuilder<R, RTags, ResultType> {
    pub fn create(
        query: SelectSource,
        order_by_items: Vec<OrderByItem>,
        view: ExprViewBoxWithTag<R, RTags>,
        alias_generator: AliasGenerator,
    ) -> Self {
        MappedQueryBuilder {
            query,
            order_by_items,
            view,
            alias_generator,
            limit: None,
            offset: 0,
            _result_ty: Default::default(),
        }
    }

    #[must_use]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;

        self
    }
}

impl<R: Value, RTags: TagList> MappedQueryBuilder<R, RTags, MultiRows> {
    #[must_use]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    #[must_use]
    pub fn first(self) -> MappedQueryBuilder<R, RTags, SingleRow> {
        self.get(1)
    }

    pub fn get(self, idx: usize) -> MappedQueryBuilder<R, RTags, SingleRow> {
        MappedQueryBuilder {
            query: self.query,
            order_by_items: self.order_by_items,
            view: self.view,
            alias_generator: self.alias_generator,
            limit: Some(idx),
            offset: self.offset,
            _result_ty: Default::default(),
        }
    }
}

impl<R: Value, RTags: TagList, ResultType: ExecuteResultType, DB: Database> Executable<R, DB>
    for MappedQueryBuilder<R, RTags, ResultType>
where
    SelectQuery: YukinoQuery<DB>,
{
    type ResultType = ResultType;
    type Query = SelectQuery;

    fn generate_query(self) -> Self::Query {
        SelectQuery::create(
            self.query,
            self.alias_generator
                .generate_select_list(self.view.collect_expr(), true),
            self.order_by_items,
            self.limit,
            self.offset,
        )
    }
}

impl<T: Value<L = U1>, TTags: TagList, ResultType: ExecuteResultType> SubqueryView<T>
    for MappedQueryBuilder<T, TTags, ResultType>
{
    fn subquery(&self) -> SelectQuery {
        SelectQuery::create(
            self.query.clone(),
            self.alias_generator
                .generate_select_list(self.view.collect_expr(), false),
            self.order_by_items.clone(),
            None,
            0,
        )
    }
}

impl<T: Value<L = U1>, TTags: TagList> ExprView<T> for MappedQueryBuilder<T, TTags, SingleRow> {
    type Tags = TTags;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBox<T>
    where
        Self: Sized,
    {
        unreachable!("QueryResultMap::from_exprs should never be called")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags> {
        Box::new(MappedQueryBuilder::create(
            self.query.clone(),
            self.order_by_items.clone(),
            self.view.expr_clone(),
            self.alias_generator.clone(),
        ))
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        arr![Expr; Expr::Subquery(self.subquery())]
    }
}

impl<T: Value<L = U1>, TTags: TagList> SubqueryIntoView<T> for MappedQueryBuilder<T, TTags, SingleRow> {
    fn as_expr(&self) -> ExprViewBox<T> {
        T::view_from_exprs(arr![Expr; Expr::Subquery(self.subquery())])
    }
}

impl<T: Value<L = U1>, TTags: TagList> SingleRowSubqueryView<T>
    for MappedQueryBuilder<T, TTags, SingleRow>
{
}
