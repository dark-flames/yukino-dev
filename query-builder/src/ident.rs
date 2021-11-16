use crate::Expr;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ident {
    pub seg: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub name: String,
}

impl Alias {
    pub fn create_ident(&self, column: &str) -> Ident {
        Ident {
            seg: vec![self.name.clone(), column.to_string()],
        }
    }

    pub fn create_ident_expr(&self, column: &str) -> Expr {
        Expr::Ident(self.create_ident(column))
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.seg.join("."))
    }
}

pub struct AliasedTable {
    pub table: String,
    pub alias: Alias,
}
