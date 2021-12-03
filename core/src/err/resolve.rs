pub use thiserror::Error;

use crate::err::YukinoError;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("Error occurs while reading schema file: `{0}`")]
    FsError(String),
    #[error("GlobInPathIsNotSupported: Glob in path({0}) is not supported")]
    GlobInPathIsNotSupported(String),
    #[error("UnsupportedSyntaxBlock: Schema file only support `struct` and `use` block")]
    UnsupportedSyntaxBlock,
    #[error("UnsupportedEntityStructType: Field of interface struct must be named field")]
    UnsupportedEntityStructType,
    #[error("NoSuitableResolveSeed: No suitable resolve seed for field `{0}`")]
    NoSuitableResolveSeed(String),
    #[error("FileParseError: Parse error occur while parse file `{0}`: {1}")]
    FileParseError(String, String),
    #[error("EntityParseError: Parse error occur while parse interface `{0}`: {1}")]
    EntityParseError(String, String),
    #[error("NoEntityAttribute: Can not find a Entity attribute on interface `{0}`")]
    NoEntityAttribute(String),
    #[error("IndexedFieldNotFound: Field `{0}` of index `{1}` was not found in interface")]
    IndexedFieldNotFound(String, String),
    #[error("NoPrimaryField: No primary field was found in interface `{0}`")]
    NoPrimaryField(String),
    #[error("FieldParseError: Parse error occur while parse field `{0}`: {1}")]
    FieldParseError(String, String),
}

impl YukinoError for ResolveError {}
