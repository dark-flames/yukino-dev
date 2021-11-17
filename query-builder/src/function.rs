use crate::{Expr, ExprMutVisitor, ExprNode, ExprVisitor};
use interface::DatabaseType;
use std::fmt::{Debug, Display, Formatter};

pub type FunctionBox = Box<dyn Function>;

pub trait Function: Debug + Display {
    fn box_clone(&self) -> FunctionBox;
}

#[derive(Debug)]
pub struct FunctionCall {
    pub func: FunctionBox,
    pub params: Vec<Expr>,
    pub return_ty: DatabaseType,
}

impl Clone for FunctionCall {
    fn clone(&self) -> Self {
        FunctionCall {
            func: self.func.box_clone(),
            params: self.params.clone(),
            return_ty: self.return_ty,
        }
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
