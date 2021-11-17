use crate::{ExprMutVisitor, ExprNode, ExprVisitor};
use interface::DatabaseType;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use time::{Date, PrimitiveDateTime, Time};

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

    Character(char),
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
            DatabaseValue::Character(v) => write!(f, "'{}'", v),
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
            DatabaseValue::Character(_) => DatabaseType::Character,
            DatabaseValue::String(_) => DatabaseType::String,
            #[cfg(any(feature = "json"))]
            DatabaseValue::Json(_) => DatabaseType::Json,
            DatabaseValue::Null(ty) => *ty,
        }
    }
}

impl ExprNode for DatabaseValue {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_lit(self)
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_lit(self)
    }
}
