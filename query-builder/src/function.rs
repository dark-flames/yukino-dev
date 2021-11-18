use crate::{Expr, ExprMutVisitor, ExprNode, ExprVisitor};
use std::fmt::{Debug, Display, Formatter};

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

impl ExprNode for FunctionCall {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_function_call(self);
        self.params.iter().for_each(|p| p.apply(visitor))
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_function_call(self);
        self.params.iter_mut().for_each(|p| p.apply_mut(visitor))
    }
}
