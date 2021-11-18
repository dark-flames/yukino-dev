use crate::{Expr, ExprMutVisitor, ExprNode, ExprVisitor};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ident {
    pub seg: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct AliasedTable {
    pub table: String,
    pub alias: Alias,
}

impl Alias {
    pub fn create_ident(&self, column: &str) -> Ident {
        let mut ident = self.single_seg_ident();
        ident.append_str(column);
        ident
    }

    pub fn create_ident_expr(&self, column: &str) -> Expr {
        Expr::Ident(self.create_ident(column))
    }

    pub fn single_seg_ident(&self) -> Ident {
        Ident {
            seg: vec![self.name.clone()],
        }
    }
}

impl Ident {
    pub fn append_str(&mut self, column: &str) {
        self.seg.push(column.to_string())
    }

    pub fn extend(&mut self, ident: Ident) {
        self.seg.extend(ident.seg);
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.seg.join("."))
    }
}

impl ExprNode for Ident {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_ident(self)
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_ident(self)
    }
}
