use std::ops::{Add, Sub};

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;
use sqlx::Database;

use query_builder::{DatabaseValue, Expr, Query, ResultRow, SelectQuery, SelectSource};

use crate::err::{RuntimeResult, YukinoError};
use crate::query::{AliasGenerator, Executable, Map, QueryResultMap};
use crate::query::exec::SingleRow;
use crate::view::{
    AggregateViewTag, ConcreteList, ExprView, ExprViewBox, ExprViewBoxWithTag, InList, MergeList,
    SingleRowSubqueryView, SubqueryIntoView, SubqueryView, TagList, TagsOfValueView, Value,
    ValueCountOf,
};

pub trait FoldResult: 'static + Clone + Send + Sync {
    type Value: Value;
    type Tags: TagList;
    fn collect_fold_expr_vec(&self) -> Vec<Expr>;

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags>;
}

pub struct FoldQueryResult<View: FoldResult> {
    query: SelectSource,
    view: View,
    alias_generator: AliasGenerator,
}

impl<View: FoldResult> FoldQueryResult<View> {
    pub fn create(query: SelectSource, view: View, alias_generator: AliasGenerator) -> Self {
        FoldQueryResult {
            query,
            view,
            alias_generator,
        }
    }
}

impl<View: FoldResult> Map<View> for FoldQueryResult<View> {
    type ResultType = SingleRow;

    fn map<R: Value, RTags: TagList, RV: Into<ExprViewBoxWithTag<R, RTags>>, F: Fn(View) -> RV>(
        self,
        f: F,
    ) -> QueryResultMap<R, RTags, Self::ResultType> {
        let result = f(self.view).into();

        QueryResultMap::create(self.query, vec![], result, self.alias_generator)
    }
}

impl<View: FoldResult, DB: Database> Executable<View::Value, View::Tags, DB> for FoldQueryResult<View>
    where SelectQuery: Query<DB, ResultRow<ValueCountOf<View::Value>>> {
    type ResultType = SingleRow;
    type Query = SelectQuery;

    fn generate_query(self) -> (Self::Query, ExprViewBoxWithTag<View::Value, View::Tags>) {
        (
            SelectQuery::create(
                self.query,
                self.alias_generator
                    .generate_select_list(self.view.collect_fold_expr_vec(), true),
                vec![],
                None,
                0,
            ),
            self.view.expr_box(),
        )
    }
}

pub trait Fold<View> {
    fn fold<RV: FoldResult, F: Fn(View) -> RV>(self, f: F) -> FoldQueryResult<RV>;
}

pub trait Fold2<View1, View2> {
    fn fold<RV: FoldResult, F: Fn(View1, View2) -> RV>(self, f: F) -> FoldQueryResult<RV>;
}

impl<T: Value<L = U1>, View: FoldResult<Value = T>> SubqueryView<T> for FoldQueryResult<View> {
    fn subquery(&self) -> SelectQuery {
        SelectQuery::create(
            self.query.clone(),
            self.alias_generator
                .generate_select_list(self.view.collect_fold_expr_vec(), false),
            vec![],
            None,
            0,
        )
    }
}

impl<T: Value<L = U1>, View: FoldResult<Value = T>> ExprView<T> for FoldQueryResult<View> {
    type Tags = View::Tags;

    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> ExprViewBox<T>
    where
        Self: Sized,
    {
        unreachable!("FoldQueryResult can't be constructed from Exprs")
    }

    fn expr_clone(&self) -> ExprViewBoxWithTag<T, Self::Tags> {
        Box::new(FoldQueryResult::create(
            self.query.clone(),
            self.view.clone(),
            self.alias_generator.clone(),
        ))
    }

    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        arr![Expr; Expr::Subquery(self.subquery())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}

impl<T: Value<L = U1>, View: FoldResult<Value = T>> SingleRowSubqueryView<T>
    for FoldQueryResult<View>
{
}

impl<T: Value<L = U1>, View: FoldResult<Value = T>> SubqueryIntoView<T> for FoldQueryResult<View> {
    fn as_expr(&self) -> ExprViewBox<T> {
        T::view_from_exprs(arr![Expr; Expr::Subquery(self.subquery())])
    }
}

impl<T1: Value, T1Tags: TagList> FoldResult for ExprViewBoxWithTag<T1, T1Tags>
where
    AggregateViewTag: InList<T1Tags>,
{
    type Value = T1;
    type Tags = T1Tags;

    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.collect_expr().into_iter().collect()
    }

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags> {
        self
    }
}

impl<T1: Value, T1Tags: TagList + MergeList<T2Tags>, T2: Value, T2Tags: TagList> FoldResult
    for (
        ExprViewBoxWithTag<T1, T1Tags>,
        ExprViewBoxWithTag<T2, T2Tags>,
    )
where
    (T1, T2): Value,
    TagsOfValueView<T1>: MergeList<TagsOfValueView<T2>>,
    AggregateViewTag: InList<T1Tags> + InList<T2Tags>,
    ValueCountOf<T1>: Add<ValueCountOf<T2>, Output = ValueCountOf<(T1, T2)>>,
    ValueCountOf<(T1, T2)>: Sub<ValueCountOf<T1>, Output = ValueCountOf<T2>>,
{
    type Value = (T1, T2);
    type Tags = ConcreteList<T1Tags, T2Tags>;

    fn collect_fold_expr_vec(&self) -> Vec<Expr> {
        self.0
            .collect_expr()
            .into_iter()
            .chain(self.1.collect_expr())
            .collect()
    }

    fn expr_box(self) -> ExprViewBoxWithTag<Self::Value, Self::Tags> {
        self.into()
    }
}
