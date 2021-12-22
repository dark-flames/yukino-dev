use std::fmt::{Display, Formatter};

use iroha::ToTokens;

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
    Decimal,
    Binary,
    Time,
    Date,
    DateTime,
    String,
    Json,
}

impl Display for DatabaseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
