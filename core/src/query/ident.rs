use crate::db::ty::DatabaseType;
use crate::query::Expr;

#[derive(Clone)]
pub struct Ident {
    pub seg: Vec<String>,
    pub ty: DatabaseType,
}

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
