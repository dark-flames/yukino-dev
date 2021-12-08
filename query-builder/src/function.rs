use std::fmt::{Debug, Display, Formatter};

use crate::{Expr, OrderByItem, QueryBuildState, SelectQuery, ToSql};
use crate::drivers::{convert_group_concat, convert_normal_aggregate_fn_call, convert_subquery_fn};

#[derive(Clone, Debug, Copy)]
pub enum AggregateFunction {
    Average,
    BitAnd,
    BitOr,
    BitXor,
    Count,
    CountDistinct,
    GroupConcat,
    Max,
    Min,
}

#[derive(Clone, Debug, Copy)]
pub enum SubqueryFunction {
    Any,
    All
}

#[derive(Clone, Debug, Copy)]
pub enum Function {
    Aggregate(AggregateFunction),
    Subquery(SubqueryFunction),
}

pub trait AggregateFunctionCall: 'static + Display + FunctionCall {
    fn clone_aggr_fn_box(&self) -> Box<dyn AggregateFunctionCall>;
}

pub trait FunctionCall: 'static + Display + Debug + ToSql {
    fn clone_box(&self) -> Box<dyn FunctionCall>;
}

#[derive(Clone, Debug)]
pub struct NormalAggregateFunctionCall {
    pub function: AggregateFunction,
    pub param: Expr,
}

#[derive(Clone, Debug)]
pub struct GroupConcatFunctionCall {
    pub expr: Expr,
    pub order_by: Vec<OrderByItem>,
    pub separator: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SubqueryFunctionCall {
    pub function: SubqueryFunction,
    pub subquery: SelectQuery
}

impl Display for NormalAggregateFunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.function {
            AggregateFunction::Average => write!(f, "AVG({})", self.param),
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

impl FunctionCall for NormalAggregateFunctionCall {
    fn clone_box(&self) -> Box<dyn FunctionCall> {
        Box::new(self.clone())
    }
}

impl AggregateFunctionCall for NormalAggregateFunctionCall {
    fn clone_aggr_fn_box(&self) -> Box<dyn AggregateFunctionCall> {
        Box::new(self.clone())
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

impl FunctionCall for GroupConcatFunctionCall {
    fn clone_box(&self) -> Box<dyn FunctionCall> {
        Box::new(self.clone())
    }
}

impl AggregateFunctionCall for GroupConcatFunctionCall {
    fn clone_aggr_fn_box(&self) -> Box<dyn AggregateFunctionCall> {
        Box::new(self.clone())
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

impl FunctionCall for SubqueryFunctionCall {
    fn clone_box(&self) -> Box<dyn FunctionCall> {
        Box::new(self.clone())
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

impl Clone for Box<dyn FunctionCall> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn AggregateFunctionCall> {
    fn clone(&self) -> Self {
        self.clone_aggr_fn_box()
    }
}
