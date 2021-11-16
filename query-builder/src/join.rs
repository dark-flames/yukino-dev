use crate::{AliasedTable, Expr};

pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum JoinType {
    LeftJoin,
    RightJoin,
    InnerJoin,
}
