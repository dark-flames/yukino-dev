use crate::err::YukinoError;
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("Error occurs while reading schema file: `{0}`")]
    FsError(String),
    #[error("GlobInPathIsNotSupported: Glob in path({0}) is not supported")]
    GlobInPathIsNotSupported(String),
    #[error("UnsupportedSyntaxBlock: Schema file only support `struct` and `use` block")]
    UnsupportedSyntaxBlock,
    #[error("UnsupportedEntityStructType: Field of entity struct must be named field")]
    UnsupportedEntityStructType,
    #[error("NoSuitableResolveCell: No suitable resolve cell for field `{0}`")]
    NoSuitableResolveCell(String),
    #[error("ParseError: Parse error occur while parse file `{0}`: String")]
    ParseError(String, String),
}

impl YukinoError for ResolveError {}
