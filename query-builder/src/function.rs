use std::fmt::{Debug, Display, Formatter, Write};

use crate::{Expr, QueryBuildState, ToSql};

#[derive(Clone, Debug)]
pub enum AggregateFunction {
    Average,
    BitAnd,
    BitOr,
    BitXor,
    Count,
    CountDistinct,
    Concat,
    Max,
    Min,
}

#[derive(Clone, Debug)]
pub enum Function {
    Aggregate(AggregateFunction),
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub func: Function,
    pub params: Vec<Expr>,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.func,
            self.params
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl ToSql for Function {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        write!(state, "{}", self)
    }
}
