use crate::db::ty::DatabaseType;
use crate::query::TypedExpr;
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

    pub fn create_ident_expr(&self, column: &str, ty: DatabaseType) -> TypedExpr {
        TypedExpr::ident(self.create_ident(column, ty)).unwrap()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.seg.join("."))
    }
}
