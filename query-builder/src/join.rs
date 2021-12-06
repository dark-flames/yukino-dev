use std::fmt::{Display, Formatter};

use crate::{AliasedTable, Expr};

pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
}

pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}

impl Display for JoinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinType::InnerJoin => write!(f, "INNER JOIN"),
            JoinType::LeftJoin => write!(f, "LEFT JOIN"),
            JoinType::RightJoin => write!(f, "RIGHT JOIN"),
        }
    }
}

impl Display for Join {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ON {}", self.ty, self.table, self.on)
    }
}
