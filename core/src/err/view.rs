use crate::view::View;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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
