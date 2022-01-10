use std::env;

use sqlx::{Executor, MySql, MySqlPool};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Decimal;

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
pub struct Meeting {
    #[id]
    pub id: u32,
    pub title: String,
    #[belongs_to(Person)]
    pub host_id: u32,
    #[belongs_to(Person)]
    pub co_host_id: u32,
    pub start_time: u64,
    pub end_time: u64,
}

pub async fn adult_hosted_meeting_length<'c, E: Executor<'c, Database = MySql>>(
    executor: E,
) -> Vec<u64> {
    let adult = Person::all().filter(|p| bt!(p.age, 18));

    Meeting::belonging_to_query::<meeting::host_id>(adult)
        .sort(|m| m.id.asc())
        .map(|m| m.end_time - m.start_time)
        .exec(executor)
        .await
        .unwrap()
}

pub async fn meeting_count_by_level<'c, E: Executor<'c, Database = MySql>>(
    executor: E,
) -> Vec<(u16, Option<Decimal>)> {
    Person::all()
        .group_by(|p| p.level)
        .fold_group(|p| {
            p.map(|p| {
                Meeting::belonging_to_view::<meeting::host_id>(&p)
                    .fold(|m| m.id.count())
                    .as_expr()
            })
            .sum()
        })
        .exec(executor)
        .await
        .unwrap()
}

pub async fn person_and_hosted_meeting(executor: &MySqlPool) -> Vec<(Person, Vec<Meeting>)> {
    let persons: Vec<Person> = Person::all().exec(executor).await.unwrap();
    let meetings = Meeting::belonging_to::<meeting::host_id>(&persons)
        .exec(executor)
        .await
        .unwrap();

    persons.join::<meeting::host_id>(meetings)
}

pub async fn hosted_meeting_titles<'c, E: Executor<'c, Database = MySql>>(
    executor: E,
) -> Vec<(u32, Option<String>)> {
    Person::all()
        .map(|p| {
            (
                p.id.clone(),
                Meeting::belonging_to_view::<meeting::host_id>(&p)
                    .fold(|m| m.sort(|m| m.id.asc()).map(|m| m.title).join(Some(", ")))
                    .as_expr(),
            )
        })
        .exec(executor)
        .await
        .unwrap()
}

pub async fn prepare_data(pool: &MySqlPool) {
    let person_list = vec![
        Person {
            id: 1,
            name: "Alice".to_string(),
            age: 15,
            level: 1,
        },
        Person {
            id: 2,
            name: "Bob".to_string(),
            age: 19,
            level: 1,
        },
        Person {
            id: 3,
            name: "Carol".to_string(),
            age: 20,
            level: 2,
        },
        Person {
            id: 4,
            name: "David".to_string(),
            age: 17,
            level: 2,
        },
    ];
    let meeting_list = vec![
        Meeting {
            id: 1,
            title: "Meeting 1".to_string(),
            host_id: 1,
            co_host_id: 2,
            start_time: 1,
            end_time: 10,
        },
        Meeting {
            id: 2,
            title: "Meeting 2".to_string(),
            host_id: 1,
            co_host_id: 2,
            start_time: 2,
            end_time: 11,
        },
        Meeting {
            id: 3,
            title: "Meeting 3".to_string(),
            host_id: 2,
            co_host_id: 3,
            start_time: 3,
            end_time: 12,
        },
        Meeting {
            id: 4,
            title: "Meeting 4".to_string(),
            host_id: 3,
            co_host_id: 4,
            start_time: 2,
            end_time: 11,
        },
        Meeting {
            id: 5,
            title: "Meeting 5".to_string(),
            host_id: 3,
            co_host_id: 4,
            start_time: 3,
            end_time: 12,
        },
    ];

    person_list.insert_all().exec(pool).await.unwrap();
    meeting_list.insert_all().exec(pool).await.unwrap();
}

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    let url = env::var("DB").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await?;

    Person::all().delete().exec(&pool).await.unwrap();
    Meeting::all().delete().exec(&pool).await.unwrap();

    prepare_data(&pool).await;

    assert_eq!(adult_hosted_meeting_length(&pool).await, vec![9, 9, 9]);

    assert_eq!(
        meeting_count_by_level(&pool).await,
        vec![(1, Some(Decimal::from(3))), (2, Some(Decimal::from(2)))]
    );

    assert_eq!(
        hosted_meeting_titles(&pool).await,
        vec![
            (1, Some("Meeting 1, Meeting 2".to_string())),
            (2, Some("Meeting 3".to_string())),
            (3, Some("Meeting 4, Meeting 5".to_string())),
            (4, None),
        ]
    );

    assert_eq!(
        person_and_hosted_meeting(&pool)
            .await
            .into_iter()
            .map(|(person, meetings)| (person.id, meetings.into_iter().map(|m| m.id).collect()))
            .collect::<Vec<(u32, Vec<u32>)>>(),
        vec![(1, vec![1, 2]), (2, vec![3]), (3, vec![4, 5]), (4, vec![]),]
    );

    Ok(())
}
