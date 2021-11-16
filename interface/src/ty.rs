use iroha::ToTokens;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, ToTokens, Debug, Eq, PartialEq, Hash)]
#[Iroha(mod_path = "yukino")]
pub enum DatabaseType {
    Bool,
    SmallInteger,
    UnsignedSmallInteger,
    Integer,
    UnsignedInteger,
    BigInteger,
    UnsignedBigInteger,
    Float,
    Double,
    Binary,
    #[cfg(any(feature = "data-time"))]
    Time,
    #[cfg(any(feature = "data-time"))]
    Date,
    #[cfg(any(feature = "data-time"))]
    DateTime,
    Character,
    String,
    #[cfg(any(feature = "json"))]
    Json,
}

impl Display for DatabaseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
