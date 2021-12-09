use std::fmt::{Display, Formatter, Write};

use crate::{AliasedTable, Expr, QueryBuildState, ToSql};

#[derive(Copy, Clone, Debug)]
pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
}

#[derive(Clone, Debug)]
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

impl ToSql for JoinType {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        match self {
            JoinType::InnerJoin => write!(state, "INNER JOIN"),
            JoinType::LeftJoin => write!(state, "LEFT JOIN"),
            JoinType::RightJoin => write!(state, "RIGHT JOIN"),
        }
    }
}

impl ToSql for Join {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        self.ty.to_sql(state)?;
        self.table.to_sql(state)?;
        write!(state, "ON")?;
        self.on.to_sql(state)
    }
}
