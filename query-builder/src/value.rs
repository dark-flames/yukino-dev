use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde_json::Value;
use sqlx::{Database, Encode, Type};
use sqlx::database::HasValueRef;
use sqlx::types::Decimal;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};

use interface::DatabaseType;

use crate::{BindArgs, QueryBuildState, QueryOf, ToSql};

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
    Decimal(Decimal),

    Binary(Binary),

    Time(Time),
    Date(Date),
    DateTime(PrimitiveDateTime),

    String(String),

    Json(Value),
    Null(DatabaseType),
}

impl Default for DatabaseValue {
    fn default() -> Self {
        DatabaseValue::Bool(false)
    }
}

pub trait AppendToArgs<'q, DB: Database> {
    fn bind_on(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB>;
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
            DatabaseValue::Decimal(v) => v.fmt(f),
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

impl Unpin for DatabaseValue {}

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
            DatabaseValue::Decimal(_) => DatabaseType::Decimal,
            DatabaseValue::Binary(_) => DatabaseType::Binary,
            DatabaseValue::Time(_) => DatabaseType::Time,
            DatabaseValue::Date(_) => DatabaseType::Date,
            DatabaseValue::DateTime(_) => DatabaseType::DateTime,
            DatabaseValue::String(_) => DatabaseType::String,
            DatabaseValue::Json(_) => DatabaseType::Json,
            DatabaseValue::Null(ty) => *ty,
        }
    }
}

impl ToSql for DatabaseValue {
    fn to_sql(&self, state: &mut QueryBuildState) -> std::fmt::Result {
        state.append_param()
    }
}

impl<'q, DB: Database> BindArgs<'q, DB> for DatabaseValue
where
    DatabaseValue: for<'p> AppendToArgs<'p, DB>,
{
    fn bind_args(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        self.bind_on(query)
    }
}

impl<'q, DB: Database> AppendToArgs<'q, DB> for DatabaseValue
where
    bool: Encode<'q, DB> + Type<DB>,
    Option<bool>: Encode<'q, DB> + Type<DB>,
    u16: Encode<'q, DB> + Type<DB>,
    Option<u16>: Encode<'q, DB> + Type<DB>,
    i16: Encode<'q, DB> + Type<DB>,
    Option<i16>: Encode<'q, DB> + Type<DB>,
    u32: Encode<'q, DB> + Type<DB>,
    Option<u32>: Encode<'q, DB> + Type<DB>,
    i32: Encode<'q, DB> + Type<DB>,
    Option<i32>: Encode<'q, DB> + Type<DB>,
    u64: Encode<'q, DB> + Type<DB>,
    Option<u64>: Encode<'q, DB> + Type<DB>,
    i64: Encode<'q, DB> + Type<DB>,
    Option<i64>: Encode<'q, DB> + Type<DB>,
    f32: Encode<'q, DB> + Type<DB>,
    Option<f32>: Encode<'q, DB> + Type<DB>,
    f64: Encode<'q, DB> + Type<DB>,
    Option<f64>: Encode<'q, DB> + Type<DB>,
    Decimal: Encode<'q, DB> + Type<DB>,
    Option<Decimal>: Encode<'q, DB> + Type<DB>,
    Time: Encode<'q, DB> + Type<DB>,
    Option<Time>: Encode<'q, DB> + Type<DB>,
    Date: Encode<'q, DB> + Type<DB>,
    Option<Date>: Encode<'q, DB> + Type<DB>,
    PrimitiveDateTime: Encode<'q, DB> + Type<DB>,
    Option<PrimitiveDateTime>: Encode<'q, DB> + Type<DB>,
    Value: Encode<'q, DB> + Type<DB>,
    Option<Value>: Encode<'q, DB> + Type<DB>,
    String: Encode<'q, DB> + Type<DB>,
    Option<String>: Encode<'q, DB> + Type<DB>,
    Binary: Encode<'q, DB> + Type<DB>,
    Option<Binary>: Encode<'q, DB> + Type<DB>,
{
    fn bind_on(self, query: QueryOf<'q, DB>) -> QueryOf<'q, DB> {
        match self {
            DatabaseValue::Bool(b) => query.bind(b),
            DatabaseValue::SmallInteger(i) => query.bind(i),
            DatabaseValue::UnsignedSmallInteger(i) => query.bind(i),
            DatabaseValue::Integer(i) => query.bind(i),
            DatabaseValue::UnsignedInteger(i) => query.bind(i),
            DatabaseValue::BigInteger(i) => query.bind(i),
            DatabaseValue::UnsignedBigInteger(i) => query.bind(i),
            DatabaseValue::Float(f) => query.bind(f),
            DatabaseValue::Double(f) => query.bind(f),
            DatabaseValue::Decimal(f) => query.bind(f),
            DatabaseValue::Binary(b) => query.bind(b),
            DatabaseValue::Time(t) => query.bind(t),
            DatabaseValue::Date(t) => query.bind(t),
            DatabaseValue::DateTime(t) => query.bind(t),
            DatabaseValue::String(s) => query.bind(s),
            DatabaseValue::Json(j) => query.bind(j),
            DatabaseValue::Null(t) => {
                fn null_of<T>() -> Option<T> {
                    None
                }
                match t {
                    DatabaseType::Bool => query.bind(null_of::<i16>()),
                    DatabaseType::SmallInteger => query.bind(null_of::<i16>()),
                    DatabaseType::UnsignedSmallInteger => query.bind(null_of::<u16>()),
                    DatabaseType::Integer => query.bind(null_of::<i32>()),
                    DatabaseType::UnsignedInteger => query.bind(null_of::<u32>()),
                    DatabaseType::BigInteger => query.bind(null_of::<i64>()),
                    DatabaseType::UnsignedBigInteger => query.bind(null_of::<u64>()),
                    DatabaseType::Float => query.bind(null_of::<f32>()),
                    DatabaseType::Double => query.bind(null_of::<f64>()),
                    DatabaseType::Binary => query.bind(null_of::<Binary>()),
                    DatabaseType::Decimal => query.bind(null_of::<Decimal>()),
                    DatabaseType::Time => query.bind(null_of::<Time>()),
                    DatabaseType::Date => query.bind(null_of::<Date>()),
                    DatabaseType::DateTime => query.bind(null_of::<PrimitiveDateTime>()),
                    DatabaseType::String => query.bind(null_of::<String>()),
                    DatabaseType::Json => query.bind(null_of::<Value>()),
                }
            }
        }
    }
}

pub type ValueRefOf<'r, DB> = <DB as HasValueRef<'r>>::ValueRef;
pub type RowOf<DB> = <DB as Database>::Row;
pub type ColumnOf<DB> = <DB as Database>::Column;
