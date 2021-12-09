use std::fmt::Result;

use crate::QueryBuildState;

pub trait ToSql {
    fn to_sql(&self, state: &mut QueryBuildState) -> Result;
}