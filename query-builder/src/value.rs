use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use generic_array::{ArrayLength, GenericArray};
use serde_json::Value;
use sqlx::{ColumnIndex, Database, Decode, Encode, Error, FromRow, Row, Type, TypeInfo, ValueRef};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::error::BoxDynError;
use sqlx::query::QueryAs;
use sqlx::types::Decimal;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};

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
    Decimal(Decimal),

    Binary(Binary),

    Time(Time),
    Date(Date),
    DateTime(PrimitiveDateTime),

    String(String),

    Json(Value),
    Null(DatabaseType),
}

pub trait AppendToArgs<'q, DB: Database> {
    fn bind_on<O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments>::Arguments>;
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
        state.append_param(self)
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
    fn bind_on<O>(
        self,
        query: QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments>,
    ) -> QueryAs<'q, DB, O, <DB as HasArguments<'q>>::Arguments> {
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

#[cfg(feature = "mysql")]
impl<'r, DB: Database> Decode<'r, DB> for DatabaseValue
where
    bool: Decode<'r, DB>,
    u16: Decode<'r, DB>,
    i16: Decode<'r, DB>,
    u32: Decode<'r, DB>,
    i32: Decode<'r, DB>,
    u64: Decode<'r, DB>,
    i64: Decode<'r, DB>,
    f32: Decode<'r, DB>,
    f64: Decode<'r, DB>,
    Decimal: Decode<'r, DB>,
    Value: Decode<'r, DB>,
    String: Decode<'r, DB>,
    Date: Decode<'r, DB>,
    Time: Decode<'r, DB>,
    PrimitiveDateTime: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        match (value.type_info().name(), value.is_null()) {
            ("BOOLEAN", false) => bool::decode(value).map(DatabaseValue::Bool),
            ("BOOLEAN", true) => Ok(DatabaseValue::Null(DatabaseType::Bool)),
            ("SMALLINT UNSIGNED", false) => {
                u16::decode(value).map(DatabaseValue::UnsignedSmallInteger)
            }
            ("SMALLINT UNSIGNED", true) => {
                Ok(DatabaseValue::Null(DatabaseType::UnsignedSmallInteger))
            }
            ("SMALLINT", false) => i16::decode(value).map(DatabaseValue::SmallInteger),
            ("SMALLINT", true) => Ok(DatabaseValue::Null(DatabaseType::SmallInteger)),
            ("INT UNSIGNED", false) => u32::decode(value).map(DatabaseValue::UnsignedInteger),
            ("INT UNSIGNED", true) => Ok(DatabaseValue::Null(DatabaseType::UnsignedInteger)),
            ("INT", false) => i32::decode(value).map(DatabaseValue::Integer),
            ("INT", true) => Ok(DatabaseValue::Null(DatabaseType::Integer)),
            ("BIGINT UNSIGNED", false) => u64::decode(value).map(DatabaseValue::UnsignedBigInteger),
            ("BIGINT UNSIGNED", true) => Ok(DatabaseValue::Null(DatabaseType::UnsignedBigInteger)),
            ("BIGINT", false) => i64::decode(value).map(DatabaseValue::BigInteger),
            ("BIGINT", true) => Ok(DatabaseValue::Null(DatabaseType::BigInteger)),
            ("FLOAT", false) => f32::decode(value).map(DatabaseValue::Float),
            ("FLOAT", true) => Ok(DatabaseValue::Null(DatabaseType::Float)),
            ("DOUBLE", false) => f64::decode(value).map(DatabaseValue::Double),
            ("DOUBLE", true) => Ok(DatabaseValue::Null(DatabaseType::Double)),
            ("DECIMAL", false) => Decimal::decode(value).map(DatabaseValue::Decimal),
            ("DECIMAL", true) => Ok(DatabaseValue::Null(DatabaseType::Decimal)),
            ("JSON", false) => Value::decode(value).map(DatabaseValue::Json),
            ("JSON", true) => Ok(DatabaseValue::Null(DatabaseType::Json)),
            ("TEXT", false) => String::decode(value).map(DatabaseValue::String),
            ("TEXT", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            ("LONGTEXT", false) => String::decode(value).map(DatabaseValue::String),
            ("LONGTEXT", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            ("VARCHAR", false) => String::decode(value).map(DatabaseValue::String),
            ("VARCHAR", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            ("CHAR", false) => String::decode(value).map(DatabaseValue::String),
            ("CHAR", true) => Ok(DatabaseValue::Null(DatabaseType::String)),
            ("DATE", false) => Date::decode(value).map(DatabaseValue::Date),
            ("DATE", true) => Ok(DatabaseValue::Null(DatabaseType::Date)),
            ("TIME", false) => Time::decode(value).map(DatabaseValue::Time),
            ("TIME", true) => Ok(DatabaseValue::Null(DatabaseType::Time)),
            ("DATETIME", false) => PrimitiveDateTime::decode(value).map(DatabaseValue::DateTime),
            ("DATETIME", true) => Ok(DatabaseValue::Null(DatabaseType::DateTime)),
            (t, _) => Err(Box::new(ExecuteError::DecodeError(format!(
                "Unsupported DB type {}",
                t
            )))),
        }
    }
}

pub struct ResultRow<L: ArrayLength<DatabaseValue>> {
    values: GenericArray<DatabaseValue, L>,
}

impl<L: ArrayLength<DatabaseValue>> From<GenericArray<DatabaseValue, L>> for ResultRow<L> {
    fn from(values: GenericArray<DatabaseValue, L>) -> Self {
        ResultRow { values }
    }
}

impl<L: ArrayLength<DatabaseValue>> From<ResultRow<L>> for GenericArray<DatabaseValue, L> {
    fn from(r: ResultRow<L>) -> Self {
        r.values
    }
}

impl<'r, DB: Database, R: Row<Database = DB>, L: ArrayLength<DatabaseValue>> FromRow<'r, R>
    for ResultRow<L>
where
    for<'n> &'n str: ColumnIndex<R>,
    DatabaseValue: Decode<'r, DB>,
{
    fn from_row(row: &'r R) -> Result<Self, Error> {
        GenericArray::from_exact_iter(
            (0..L::to_usize())
                .into_iter()
                .map(|index| {
                    let name = format!("result_{}", index);
                    let value_ref = row.try_get_raw(name.as_str()).unwrap();
                    let result = DatabaseValue::decode(value_ref)
                        .map_err(Error::Decode)
                        .unwrap();

                    Ok(result)
                })
                .collect::<Result<Vec<_>, Error>>()?,
        )
        .ok_or_else(|| {
            Error::Decode(Box::new(ExecuteError::ResultLengthError(
                L::to_usize(),
                row.len(),
            )))
        })
        .map(Into::into)
    }
}
