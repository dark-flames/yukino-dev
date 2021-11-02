use crate::db::ty::DatabaseType;
use crate::query::Expr;

pub type FunctionBox = Box<dyn Function>;

pub trait Function {
    fn box_clone(&self) -> FunctionBox;
}

pub struct FunctionCall {
    func: FunctionBox,
    params: Vec<Expr>,
    return_ty: DatabaseType,
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
