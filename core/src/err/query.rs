use crate::err::YukinoError;
use query_builder::DatabaseType;
use query_builder::Expr;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use thiserror::Error;

pub type ExprResult<T> = Result<T, ExprError>;

#[derive(Debug)]
pub struct ExprError {
    expr: String,
    msg: String,
}

pub trait ErrorOnExpr: Error {
    fn as_expr_err(&self, expr: &Expr) -> ExprError {
        ExprError {
            expr: expr.to_string(),
            msg: self.to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("UnimplementedOperator: Operator `{0}` not implemented on type `{1}`")]
    UnimplementedOperator(String, DatabaseType),
    #[error("CannotApplyOperator: Operator `{0}` can not be applied on type `{1}` and `{2}`")]
    CannotApplyOperator(String, DatabaseType, DatabaseType),
}

impl Display for ExprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Some error occur on expr `{}`: {}", self.expr, self.msg)
    }
}

impl Error for ExprError {}

impl YukinoError for ExprError {}

impl ErrorOnExpr for TypeError {}
