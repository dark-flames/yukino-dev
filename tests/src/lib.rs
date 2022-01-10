use std::time::SystemTime;

use sqlx::types::Decimal;
use sqlx::types::time::PrimitiveDateTime;

use yukino::Entity;

#[derive(Entity, Debug, Clone)]
pub struct Bar {
    #[belongs_to(Foo)]
    pub foo_id: u32,
    pub name: String,
}

#[derive(Entity, Clone, Debug)]
pub struct Foo {
    #[id]
    #[auto_increment]
    pub id: u32,
    pub boolean: bool,
    pub u_short: u16,
    pub short: i16,
    pub u_int: u32,
    pub int: i32,
    pub u_long: u64,
    pub long: i64,
    pub float: f32,
    pub double: f64,
    pub string: String,
    pub optional: Option<u32>,
    pub decimal: sqlx::types::Decimal,
    pub optional_decimal: Option<sqlx::types::Decimal>,
    pub date: sqlx::types::time::Date,
    pub time: sqlx::types::time::Time,
    pub datetime: sqlx::types::time::PrimitiveDateTime,
}

pub fn create_foo() -> Foo {
    let now: PrimitiveDateTime = SystemTime::now().into();
    Foo {
        id: 114514,
        boolean: false,
        u_short: 0,
        short: 0,
        u_int: 0,
        int: 0,
        u_long: 0,
        long: 0,
        float: 0.0,
        double: 0.0,
        string: "".to_string(),
        optional: None,
        decimal: Decimal::new(0, 0),
        optional_decimal: None,
        date: now.date(),
        time: now.time(),
        datetime: now,
    }
}

pub fn create_new_foo() -> NewFoo {
    let now: PrimitiveDateTime = SystemTime::now().into();
    NewFoo {
        boolean: false,
        u_short: 0,
        short: 0,
        u_int: 0,
        int: 0,
        u_long: 0,
        long: 0,
        float: 0.0,
        double: 0.0,
        string: "".to_string(),
        optional: None,
        decimal: Decimal::new(0, 0),
        optional_decimal: None,
        date: now.date(),
        time: now.time(),
        datetime: now,
    }
}
