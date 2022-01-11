use std::fmt::Display;

use sqlx::Database;

use crate::{BindArgs, ToSql};

pub trait Query<DB: Database, O>: ToSql + Display + Send + Sync
where
    Self: for<'q> BindArgs<'q, DB, O>,
{
}
