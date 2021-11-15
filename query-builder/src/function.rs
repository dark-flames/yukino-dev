use crate::{DatabaseType, Expr};
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
