use crate::err::YukinoError;
use crate::view::View;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Debug)]
pub struct ViewError {
    msg: String,
    view: String,
}

pub trait ErrorOnView: Error {
    fn as_view_err<T>(&self, view: &dyn View<T>) -> ViewError {
        ViewError {
            msg: self.to_string(),
            view: format!("{:?}", view),
        }
    }
}

impl Display for ViewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Some error occur on view `{}`: {}", self.view, self.msg)
    }
}

impl Error for ViewError {}

#[derive(Error, Debug)]
pub enum ViewResolveError {
    #[error("UnexpectedParamCount: Parameters of calculation must be 2, got {0}.")]
    UnexpectedCalculationParamCount(usize),
    #[error("UnexpectedParamPatternType: This parameter expected to be a `{0}`.")]
    UnexpectedParamPatternType(String),
    #[error("RefIsInvalid: `ref` at here is not supported.")]
    RefIsInvalid,
    #[error("SubPatternIsInvalid: Sub-pattern at here is not supported.")]
    SubPatternIsInvalid,
    #[error("MutableIsInvalid: `mut` at here is not supported.")]
    MutableIsInvalid,
    #[error("NotTwoElementsTuple: Only two elements tuple is supported.")]
    NotTwoElementsTuple,
    #[error("IdentConflict: More than one function parameter have the same name.")]
    IdentConflict,
    #[error("CannotUnwrap: Cannot unwrap view into this pattern.")]
    CannotUnwrap,
    #[error("UnsupportedExpr: This expression is not supported here.")]
    UnsupportedExpr,
}

impl YukinoError for ViewResolveError {}
