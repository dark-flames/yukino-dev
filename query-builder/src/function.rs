use std::fmt::Debug;

use crate::{Expr, OrderByItem};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Function {
    Aggregate(AggregateFunction),
}

pub trait FunctionCall: Debug {
    fn boxed(&self) -> Box<dyn FunctionCall>;
}

pub trait AggregateFunctionCall: FunctionCall {}

#[derive(Debug, Clone)]
pub struct SingleArgumentAggregateFunctionCall {
    pub func: AggregateFunction,
    pub arg: Expr
}

#[derive(Debug, Clone)]
pub struct GroupConcat {
    pub expr: Expr,
    pub order_by: Vec<OrderByItem>,
    pub separator: String,
}

impl FunctionCall for SingleArgumentAggregateFunctionCall {
    fn boxed(&self) -> Box<dyn FunctionCall> {
        Box::new(self.clone())
    }
}

impl AggregateFunctionCall for SingleArgumentAggregateFunctionCall {}

impl FunctionCall for GroupConcat {
    fn boxed(&self) -> Box<dyn FunctionCall> {
        Box::new(self.clone())
    }
}

impl AggregateFunctionCall for GroupConcat {}

macro_rules! single_arg_aggr_func {
    ($name: ident, $variant: ident) => {
        pub fn $name(arg: Expr) -> SingleArgumentAggregateFunctionCall {
            SingleArgumentAggregateFunctionCall {
                func: AggregateFunction::$variant,
                arg,
            }
        }
    }
}

single_arg_aggr_func!(average, Average);
single_arg_aggr_func!(bit_and, BitAnd);
single_arg_aggr_func!(bit_or, BitOr);
single_arg_aggr_func!(bit_xor, BitXor);
single_arg_aggr_func!(count, Count);
single_arg_aggr_func!(count_distinct, CountDistinct);
single_arg_aggr_func!(max, Max);
single_arg_aggr_func!(min, Min);

pub fn group_concat(expr: Expr, order_by: Vec<OrderByItem>, separator: String) -> GroupConcat {
    GroupConcat {
        expr,
        order_by,
        separator,
    }
}
