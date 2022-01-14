#[macro_use]
extern crate diesel;

pub use interface::*;

pub mod diesel_benches;
pub mod sea_orm_benches;
pub mod sqlx_benches;
pub mod yukino_benches;

mod diesel_schema;
mod interface;
