use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde_json::Value;
use sqlx::{Decode, Encode, TypeInfo, ValueRef};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
#[cfg(feature = "mysql")]
use sqlx::MySql;
use time::{Date, PrimitiveDateTime, Time};

use interface::DatabaseType;

use crate::{ExecuteError, QueryBuildState, ToSql};

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

    String(String),

    #[cfg(any(feature = "json"))]
    Json(Value),
    Null(DatabaseType),
}

impl Display for DatabaseValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseValue::Bool(v) => v.fmt(f),
            DatabaseValue::SmallInteger(v) => v.fmt(f),
            DatabaseValue::UnsignedSmallInteger(v) => v.fmt(f),
            DatabaseValue::Integer(v) => v.fmt(f),
            DatabaseValue::UnsignedInteger(v) => v.fmt(f),
            DatabaseValue::BigInteger(v) => v.fmt(f),
            DatabaseValue::UnsignedBigInteger(v) => v.fmt(f),
            DatabaseValue::Float(v) => v.fmt(f),
            DatabaseValue::Double(v) => v.fmt(f),
            DatabaseValue::Binary(_) => write!(f, "BinaryData"),
            DatabaseValue::Time(v) => v.fmt(f),
            DatabaseValue::Date(v) => v.fmt(f),
            DatabaseValue::DateTime(v) => v.fmt(f),
            DatabaseValue::String(v) => write!(f, "\"{}\"", v),
            DatabaseValue::Json(v) => v.fmt(f),
            DatabaseValue::Null(_) => write!(f, "NULL"),
        }
    }
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
            DatabaseValue::String(_) => DatabaseType::String,
            #[cfg(any(feature = "json"))]
            DatabaseValue::Json(_) => DatabaseType::Json,
            DatabaseValue::Null(ty) => *ty,
        }
    }
}

impl ToSql for DatabaseValue {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        state.append_param(self)
    }
}

#[cfg(feature = "mysql")]
impl<'r> Decode<'r, MySql> for DatabaseValue
    where bool: Decode<'r, MySql>,
        u16: Decode<'r, MySql>,
        i16: Decode<'r, MySql>,
        u32: Decode<'r, MySql>,
        i32: Decode<'r, MySql>,
        u64: Decode<'r, MySql>,
        i64: Decode<'r, MySql>,
        f32: Decode<'r, MySql>,
        f64: Decode<'r, MySql>,
        Value: Decode<'r, MySql>,
        String: Decode<'r, MySql>
{
    fn decode(value: <MySql as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let type_info = value.type_info();

        match (type_info.name(), type_info.is_null()) {
            ("BOOLEAN", false) => bool::decode(value).map(DatabaseValue::Bool),
            ("BOOLEAN", true) => Ok(DatabaseValue::Null(DatabaseType::Bool)),
            ("SMALLINT UNSIGNED", false) => u16::decode(value).map(DatabaseValue::UnsignedSmallInteger),
            ("SMALLINT UNSIGNED", true) => Ok(DatabaseValue::Null(DatabaseType::UnsignedSmallInteger)),
            ("SMALLINT", false) => i16::decode(value).map(DatabaseValue::SmallInteger),
            ("SMALLINT", true) => Ok(DatabaseValue::Null(DatabaseType::SmallInteger)),
            ("INT UNSIGNED", false) => u32::decode(value).map(DatabaseValue::UnsignedInteger),
            ("INT UNSIGNED", true) => Ok(DatabaseValue::Null(DatabaseType::UnsignedInteger)),
            ("INT", false) => i32::decode(value).map(DatabaseValue::Integer),
            ("INT", true) => Ok(DatabaseValue::Null(DatabaseType::Integer)),
            ("BIGINT UNSIGNED", false) => u64::decode(value).map(DatabaseValue::UnsignedBigInteger),
            ("BIGINT UNSIGNED", true) => Ok(DatabaseValue::Null(DatabaseType::UnsignedBigInteger)),
            ("BIG INT", false) => i64::decode(value).map(DatabaseValue::BigInteger),
            ("BIG INT", true) => Ok(DatabaseValue::Null(DatabaseType::BigInteger)),
            ("FLOAT", false) => f32::decode(value).map(DatabaseValue::Float),
            ("FLOAT", true) => Ok(DatabaseValue::Null(DatabaseType::Float)),
            ("DOUBLE", false) => f64::decode(value).map(DatabaseValue::Double),
            ("DOUBLE", true) => Ok(DatabaseValue::Null(DatabaseType::Double)),
            ("JSON", false) => Value::decode(value).map(DatabaseValue::Json),
            ("JSON", true) => Ok(DatabaseValue::Null(DatabaseType::Json)),
            ("TEXT", false) => String::decode(value).map(DatabaseValue::String),
            ("TEXT", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            ("VARCHAR", false) => String::decode(value).map(DatabaseValue::String),
            ("VARCHAR", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            (_, _) => Err(Box::new(ExecuteError::DecodeError("Unsupported DB type".to_string())))
        }
    }
}

#[cfg(feature = "mysql")]
impl<'q> Encode<'q, MySql> for DatabaseValue
    where bool: Encode<'q, MySql>,
          u16: Encode<'q, MySql>,
          i16: Encode<'q, MySql>,
          u32: Encode<'q, MySql>,
          i32: Encode<'q, MySql>,
          u64: Encode<'q, MySql>,
          i64: Encode<'q, MySql>,
          f32: Encode<'q, MySql>,
          f64: Encode<'q, MySql>,
          Time: Encode<'q, MySql>,
          Date: Encode<'q, MySql>,
          PrimitiveDateTime: Encode<'q, MySql>,
          Value: Encode<'q, MySql>,
          String: Encode<'q, MySql>,
          Binary: Encode<'q, MySql>
{
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        match self {
            DatabaseValue::Bool(b) => b.encode_by_ref(buf),
            DatabaseValue::SmallInteger(i) => i.encode_by_ref(buf),
            DatabaseValue::UnsignedSmallInteger(i) => i.encode_by_ref(buf),
            DatabaseValue::Integer(i) => i.encode_by_ref(buf),
            DatabaseValue::UnsignedInteger(i) => i.encode_by_ref(buf),
            DatabaseValue::BigInteger(i) => i.encode_by_ref(buf),
            DatabaseValue::UnsignedBigInteger(i) => i.encode_by_ref(buf),
            DatabaseValue::Float(f) => f.encode_by_ref(buf),
            DatabaseValue::Double(f) => f.encode_by_ref(buf),
            DatabaseValue::Binary(b) => b.encode_by_ref(buf),
            DatabaseValue::Time(t) => t.encode_by_ref(buf),
            DatabaseValue::Date(t) => t.encode_by_ref(buf),
            DatabaseValue::DateTime(t) => t.encode_by_ref(buf),
            DatabaseValue::String(s) => s.encode_by_ref(buf),
            DatabaseValue::Json(j) => j.encode_by_ref(buf),
            DatabaseValue::Null(_) => IsNull::Yes
        }
    }
}
