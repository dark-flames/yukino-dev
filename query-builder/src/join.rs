use interface::JoinType;

use crate::{AliasedTable, Expr};

#[derive(Clone, Debug)]
pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}
