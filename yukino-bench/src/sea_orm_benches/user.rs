use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: NaiveDate,
    pub since: NaiveDateTime,
    #[sea_orm(column_type = "Text")]
    pub introduction: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::examination::Entity")]
    Posts,
}

impl Related<super::examination::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
