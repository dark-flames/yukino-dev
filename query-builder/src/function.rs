use std::fmt::{Debug, Display, Formatter};

use crate::Expr;

#[derive(Clone, Debug)]
pub enum Function {
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
