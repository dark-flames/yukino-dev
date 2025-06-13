use std::fmt::{Display, Formatter};

use quote_data::QuoteIt;

#[derive(Copy, Clone, QuoteIt, Debug, Eq, PartialEq, Hash)]
#[mod_path = "yukino"]
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
