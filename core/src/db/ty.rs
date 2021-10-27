use iroha::ToTokens;
use serde_json::Value;
use std::collections::HashMap;
#[cfg(any(feature = "data-time"))]
use time::{Date, PrimitiveDateTime, Time};

#[derive(Copy, Clone, ToTokens, Debug, Eq, PartialEq)]
#[Iroha(mod_path = "yukino::db::ty")]
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
    Timestamp,
    Character,
    String,
    Text,
    #[cfg(any(feature = "json"))]
    Json,
}

pub type Binary = Vec<u8>;

pub type ValuePack = HashMap<String, DatabaseValue>;

#[derive(Debug, Clone)]
pub enum DatabaseValue {
    Bool(bool),
    SmallInteger(i16),
    UnsignedSmallInteger(u16),
    Integer(i32),
    UnsignedInteger(u32),
    BigInteger(i64),
    UnsignedBigInteger(u64),

    Float(f32),
    Double(f64),

    Binary(Binary),

    #[cfg(any(feature = "data-time"))]
    Time(Time),
    #[cfg(any(feature = "data-time"))]
    Date(Date),
    #[cfg(any(feature = "data-time"))]
    DateTime(PrimitiveDateTime),

    Timestamp(u64),

    Character(char),
    String(String),
    Text(String),

    #[cfg(any(feature = "json"))]
    Json(Value),
    Null(DatabaseType),
}

impl From<&DatabaseValue> for DatabaseType {
    fn from(database_value: &DatabaseValue) -> Self {
        match database_value {
            DatabaseValue::Bool(_) => DatabaseType::Bool,
            DatabaseValue::SmallInteger(_) => DatabaseType::SmallInteger,
            DatabaseValue::UnsignedSmallInteger(_) => DatabaseType::UnsignedSmallInteger,
            DatabaseValue::Integer(_) => DatabaseType::UnsignedInteger,
            DatabaseValue::UnsignedInteger(_) => DatabaseType::UnsignedInteger,
            DatabaseValue::BigInteger(_) => DatabaseType::BigInteger,
            DatabaseValue::UnsignedBigInteger(_) => DatabaseType::UnsignedBigInteger,
            DatabaseValue::Float(_) => DatabaseType::Float,
            DatabaseValue::Double(_) => DatabaseType::Double,
            DatabaseValue::Binary(_) => DatabaseType::Binary,
            #[cfg(any(feature = "data-time"))]
            DatabaseValue::Time(_) => DatabaseType::Time,
            #[cfg(any(feature = "data-time"))]
            DatabaseValue::Date(_) => DatabaseType::Date,
            #[cfg(any(feature = "data-time"))]
            DatabaseValue::DateTime(_) => DatabaseType::DateTime,
            DatabaseValue::Timestamp(_) => DatabaseType::Timestamp,
            DatabaseValue::Character(_) => DatabaseType::Character,
            DatabaseValue::String(_) => DatabaseType::String,
            DatabaseValue::Text(_) => DatabaseType::Text,
            #[cfg(any(feature = "json"))]
            DatabaseValue::Json(_) => DatabaseType::Json,
            DatabaseValue::Null(ty) => *ty,
        }
    }
}
