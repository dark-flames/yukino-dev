use crate::err::YukinoError;
use crate::query::Expr;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

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

impl Display for ExprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Some error occur on expr `{}`: {}", self.expr, self.msg)
    }
}

impl Error for ExprError {}

impl YukinoError for ExprError {}
