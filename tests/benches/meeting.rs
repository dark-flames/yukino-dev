use std::env;

use criterion::{Criterion, criterion_main};
use criterion::criterion_group;
use sqlx::{Executor, MySql, MySqlPool};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Decimal;
use tokio::runtime::Runtime;

use yukino::prelude::*;

// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor

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

    for person in person_list {
        person.insert().exec(pool).await.unwrap();
    }

    for meeting in meeting_list {
        meeting.insert().exec(pool).await.unwrap();
    }
}

pub async fn delete_all(pool: &MySqlPool) {
    Person::all().delete().exec(pool).await.unwrap();
    Meeting::all().delete().exec(pool).await.unwrap();
}

pub async fn bench_group(pool: &MySqlPool) {
    delete_all(pool).await;
    prepare_data(pool).await;
    adult_hosted_meeting_length(pool).await;
    meeting_count_by_level(pool).await;
    hosted_meeting_titles(pool).await;
}

pub fn run_bench_group(c: &mut Criterion) {
    let url = env::var("DB").unwrap();
    let runtime = Runtime::new().unwrap();

    let pool = runtime
        .block_on(MySqlPoolOptions::new().max_connections(10).connect(&url))
        .unwrap();

    runtime.block_on(delete_all(&pool));
    runtime.block_on(prepare_data(&pool));

    c.bench_function("adult_hosted_meeting_length", |b| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&runtime)
            .iter(|| adult_hosted_meeting_length(&pool));
    });

    c.bench_function("meeting_count_by_level", |b| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&runtime).iter(|| meeting_count_by_level(&pool));
    });

    c.bench_function("hosted_meeting_titles", |b| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&runtime).iter(|| hosted_meeting_titles(&pool));
    });
}

criterion_group!(benches, run_bench_group);
criterion_main!(benches);
