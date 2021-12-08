use std::fmt::{Result, Write};

use crate::{AggregateFunction, GroupConcatFunctionCall, NormalAggregateFunctionCall, QueryBuildState, SubqueryFunction, SubqueryFunctionCall, ToSql};

pub fn convert_group_concat(fn_call: &GroupConcatFunctionCall, state: &mut QueryBuildState) -> Result {
    write!(state, "GROUP_CONCAT(")?;
    fn_call.expr.to_sql(state)?;
    if !fn_call.order_by.is_empty() {
        write!(state, "ORDER BY")?;
        let last_index = fn_call.order_by.len() - 1;
        for (index, item) in fn_call.order_by.iter().enumerate() {
            item.to_sql(state)?;

            if index != last_index {
                write!(state, ",")?;
            }
        }
    }

    if let Some(separator) = &fn_call.separator {
        write!(state, "SEPARATOR {}", separator)?
    }

    write!(state, ")")
}

pub fn convert_subquery_fn(fn_call: &SubqueryFunctionCall, state: &mut QueryBuildState) -> Result {
    match &fn_call.function {
        SubqueryFunction::Any => write!(state, "ANY"),
        SubqueryFunction::All => write!(state, "ALL"),
    }?;
    write!(state, "(")?;
    fn_call.subquery.to_sql(state)?;
    write!(state, ")")
}

pub fn convert_normal_aggregate_fn_call(fn_call: &NormalAggregateFunctionCall, state: &mut QueryBuildState) -> Result {
    match &fn_call.function {
        AggregateFunction::Average => write!(state, "AVG"),
        AggregateFunction::BitAnd => write!(state, "BIT_AND"),
        AggregateFunction::BitOr => write!(state, "BIT_OR"),
        AggregateFunction::BitXor => write!(state, "BIT_XOR"),
        AggregateFunction::Count|AggregateFunction::CountDistinct => write!(state, "COUNT"),
        AggregateFunction::Max => write!(state, "Max"),
        AggregateFunction::Min => write!(state, "Min"),
        _ => unreachable!()
    }?;
    write!(state, "(")?;
    if let AggregateFunction::CountDistinct = fn_call.function {
        write!(state, "DISTINCT")?;
    }

    fn_call.param.to_sql(state)?;
    write!(state, ")")
}
