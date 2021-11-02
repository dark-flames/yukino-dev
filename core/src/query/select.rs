use crate::db::ty::DatabaseType;
use crate::query::Expr;

pub struct SelectedItem {
    pub alias: String,
    pub expr: Expr,
    pub ty: DatabaseType,
}
