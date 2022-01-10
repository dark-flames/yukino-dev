use std::fmt::{Display, Formatter, Write};

use sqlx::Database;
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;

use crate::{AliasedTable, AppendToArgs, BindArgs, DatabaseValue, Expr, QueryBuildState, ToSql};

#[derive(Copy, Clone, Debug)]
pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
}

unsafe impl Send for JoinType {}
unsafe impl Sync for JoinType {}

#[derive(Clone, Debug)]
pub struct Join {
    pub ty: JoinType,
    pub table: AliasedTable,
    pub on: Expr,
}

unsafe impl Send for Join {}
unsafe impl Sync for Join {}

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

impl BindArgs for Join {
    fn bind_args<'q, DB: Database, O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>
    where
        DatabaseValue: AppendToArgs<'q, DB>,
    {
        self.on.bind_args(query)
    }
}
