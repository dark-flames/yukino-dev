use std::fmt::Display;

use sqlx::Database;

use crate::{BindArgs, ToSql};

pub trait YukinoQuery<DB: Database>: ToSql + Display + Send + Sync
where
    Self: for<'q> BindArgs<'q, DB>,
{
}
