use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum ExecuteError {}

impl YukinoError for ExecuteError {}
