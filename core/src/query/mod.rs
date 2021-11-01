use crate::db::ty::DatabaseType;

pub struct SelectedItem {
    pub alias: String,
    pub expr: String,
    pub ty: DatabaseType,
}
