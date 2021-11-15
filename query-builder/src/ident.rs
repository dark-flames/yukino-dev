use crate::{DatabaseType, Expr};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident {
    pub seg: Vec<String>,
    pub ty: DatabaseType,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub name: String,
}

impl Alias {
    pub fn create_ident(&self, column: &str, ty: DatabaseType) -> Ident {
        Ident {
            seg: vec![self.name.clone(), column.to_string()],
            ty,
        }
    }

    pub fn create_ident_expr(&self, column: &str, ty: DatabaseType) -> Expr {
        Expr::Ident(self.create_ident(column, ty))
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.seg.join("."))
    }
}
