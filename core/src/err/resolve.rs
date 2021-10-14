use crate::err::YukinoError;
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("Error occurs while reading schema file: `{0}`")]
    FsError(String),
    #[error("GlobInPathIsNotSupported: Glob in path({0}) is not supported")]
    GlobInPathIsNotSupported(String),
    #[error("NoSuitableResolveCell: No suitable resolve cell for field \"{0}\"")]
    NoSuitableResolveCell(String),
}

impl YukinoError for ResolveError {}
