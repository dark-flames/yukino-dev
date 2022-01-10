use std::fmt::{Debug, Display, Formatter};

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{
    AppendToArgs, BindArgs, DatabaseValue, Expr, OrderByItem, QueryBuildState, SelectQuery, ToSql,
};
use crate::drivers::{convert_group_concat, convert_normal_aggregate_fn_call, convert_subquery_fn};

#[derive(Clone, Debug, Copy)]
pub enum AggregateFunction {
    Average,
    Sum,
    BitAnd,
    BitOr,
    BitXor,
    Count,
    CountDistinct,
    GroupConcat,
    Max,
    Min,
}

unsafe impl Send for AggregateFunction {}
unsafe impl Sync for AggregateFunction {}

#[derive(Clone, Debug, Copy)]
pub enum SubqueryFunction {
    Any,
    All,
}

unsafe impl Send for SubqueryFunction {}
unsafe impl Sync for SubqueryFunction {}

#[derive(Clone, Debug, Copy)]
pub enum Function {
    Aggregate(AggregateFunction),
    Subquery(SubqueryFunction),
}

unsafe impl Send for Function {}
unsafe impl Sync for Function {}

#[derive(Clone, Debug)]
pub enum FunctionCall {
    Aggregate(AggregateFunctionCall),
    Subquery(SubqueryFunctionCall),
}

#[derive(Clone, Debug)]
pub enum AggregateFunctionCall {
    Normal(NormalAggregateFunctionCall),
    GroupConcat(GroupConcatFunctionCall),
}

#[derive(Clone, Debug)]
pub struct NormalAggregateFunctionCall {
    pub function: AggregateFunction,
    pub param: Expr,
}

unsafe impl Send for NormalAggregateFunctionCall {}
unsafe impl Sync for NormalAggregateFunctionCall {}

#[derive(Clone, Debug)]
pub struct GroupConcatFunctionCall {
    pub expr: Expr,
    pub order_by: Vec<OrderByItem>,
    pub separator: Option<String>,
}

unsafe impl Send for GroupConcatFunctionCall {}
unsafe impl Sync for GroupConcatFunctionCall {}

#[derive(Clone, Debug)]
pub struct SubqueryFunctionCall {
    pub function: SubqueryFunction,
    pub subquery: SelectQuery,
}

unsafe impl Send for SubqueryFunctionCall {}
unsafe impl Sync for SubqueryFunctionCall {}

impl Display for NormalAggregateFunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.function {
            AggregateFunction::Average => write!(f, "AVG({})", self.param),
            AggregateFunction::Sum => write!(f, "Sum({})", self.param),
            AggregateFunction::BitAnd => write!(f, "BIT_AND({})", self.param),
            AggregateFunction::BitOr => write!(f, "BIT_OR({})", self.param),
            AggregateFunction::BitXor => write!(f, "BIT_XOR({})", self.param),
            AggregateFunction::Count => write!(f, "COUNT({})", self.param),
            AggregateFunction::CountDistinct => write!(f, "COUNT(DISTINCT {})", self.param),
            AggregateFunction::Max => write!(f, "MAX({})", self.param),
            AggregateFunction::Min => write!(f, "MIN({})", self.param),
            _ => unreachable!("unsupported aggregate function"),
        }
    }
}

impl ToSql for NormalAggregateFunctionCall {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        convert_normal_aggregate_fn_call(self, state)
    }
}

impl BindArgs for NormalAggregateFunctionCall {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.param.bind_args(query)
    }
}

impl Display for GroupConcatFunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GROUP_CONCAT({}", self.expr)?;
        if !self.order_by.is_empty() {
            write!(f, " ORDER BY ")?;
            for (i, item) in self.order_by.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
        }
        if let Some(ref separator) = self.separator {
            write!(f, " SEPARATOR '{}'", separator)?;
        }
        write!(f, ")")
    }
}

impl ToSql for GroupConcatFunctionCall {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        convert_group_concat(self, state)
    }
}

impl BindArgs for GroupConcatFunctionCall {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.order_by.bind_args(self.expr.bind_args(query))
    }
}

impl Display for SubqueryFunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.function, self.subquery)
    }
}

impl ToSql for SubqueryFunctionCall {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        convert_subquery_fn(self, state)
    }
}

impl BindArgs for SubqueryFunctionCall {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.subquery.bind_args(query)
    }
}

impl ToSql for AggregateFunctionCall {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            AggregateFunctionCall::Normal(normal) => normal.to_sql(state),
            AggregateFunctionCall::GroupConcat(group) => group.to_sql(state),
        }
    }
}

impl BindArgs for AggregateFunctionCall {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        match self {
            AggregateFunctionCall::Normal(normal) => normal.bind_args(query),
            AggregateFunctionCall::GroupConcat(group) => group.bind_args(query),
        }
    }
}

impl ToSql for FunctionCall {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            FunctionCall::Aggregate(a) => a.to_sql(state),
            FunctionCall::Subquery(s) => s.to_sql(state),
        }
    }
}

impl BindArgs for FunctionCall {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        match self {
            FunctionCall::Aggregate(a) => a.bind_args(query),
            FunctionCall::Subquery(s) => s.bind_args(query),
        }
    }
}

impl Display for AggregateFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for SubqueryFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AggregateFunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateFunctionCall::Normal(normal) => write!(f, "{}", normal),
            AggregateFunctionCall::GroupConcat(group) => write!(f, "{}", group),
        }
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCall::Aggregate(a) => write!(f, "{}", a),
            FunctionCall::Subquery(s) => write!(f, "{}", s),
        }
    }
}
