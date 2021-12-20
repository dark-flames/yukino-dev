use sqlx::{Executor, MySql};

use yukino::Entity;
use yukino::prelude::*;

#[derive(Entity, Clone, Debug)]
pub struct Person {
    #[id]
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub level: u16,
}

#[derive(Entity, Clone, Debug)]
#[belongs_to(Person, host_id)]
pub struct Meeting {
    #[id]
    pub id: u32,
    pub title: String,
    pub host_id: u32,
    pub start_time: u64,
    pub end_time: u64,
}

pub async fn adult_hosted_meeting_length<'c, E: Executor<'c, Database = MySql>>(
    executor: E,
) -> Vec<u64> {
    let adult = Person::all().filter(|p| bt!(p.age, 18));

    Meeting::belonging_to_query(adult)
        .map(|m| m.end_time - m.start_time)
        .exec(executor)
        .await
        .unwrap()
}

pub async fn average_meeting_count_by_level<'c, E: Executor<'c, Database = MySql>>(
    executor: E,
) -> Vec<(u16, u64)> {
    Person::all()
        .group_by(|p| p.level)
        .fold_group(|p| {
            p.map(|p| {
                Meeting::belonging_to_view(&p)
                    .fold(|m| m.id.count())
                    .as_expr()
            }).average()
        }).exec(executor)
        .await
        .unwrap()
}

