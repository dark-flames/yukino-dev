use std::collections::HashMap;

use sqlx::{Connection, FromRow, MySqlConnection, query, Row};
use sqlx::types::Decimal;
use tokio::runtime::Runtime;

use crate::interface::{CommonNewUser, Handler};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: sqlx::types::time::Date,
    pub since: sqlx::types::time::PrimitiveDateTime,
    pub introduction: String,
}

#[derive(sqlx::FromRow)]
pub struct Examination {
    pub id: i32,
    pub user_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub comment: String,
}

pub struct SqlxHandler {
    runtime: Runtime,
    connection: MySqlConnection,
}

impl Handler for SqlxHandler {
    type LocalNewUser = CommonNewUser;

    fn orm_name() -> &'static str
    where
        Self: Sized,
    {
        "sqlx"
    }

    fn create(url: &'static str) -> Self
    where
        Self: Sized,
    {
        let runtime = Runtime::new().expect("fail to create tokio runtime");

        let connection = runtime.block_on(async {
            MySqlConnection::connect(url)
                .await
                .expect("fail to create connection")
        });

        SqlxHandler {
            connection,
            runtime,
        }
    }

    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>) {
        self.runtime.block_on(async {
            let mut query = "INSERT INTO user (name, age, phone, address, birthday, since, introduction) VALUES ".to_string();

            let values = users.iter().map(|_| "(?, ?, ?, ?, ?, ?, ?)").collect::<Vec<_>>().join(",");

            query += &values;


            let query = sqlx::query(&query);

            users.into_iter().fold(
                query,
                |q, user| {
                    q.bind(user.name)
                        .bind(user.age)
                        .bind(user.phone)
                        .bind(user.address)
                        .bind(user.birthday)
                        .bind(user.since)
                        .bind(user.introduction)
                }
            ).execute(&mut self.connection).await.unwrap();
        })
    }

    fn bench_fetch_all(&mut self) {
        self.runtime.block_on(async {
            sqlx::query_as::<_, User>(
                "SELECT id, name, age, phone, address, birthday, since, introduction FROM user",
            )
            .fetch_all(&mut self.connection)
            .await
            .unwrap();
        })
    }

    fn bench_zip_association(&mut self) {
        self.runtime.block_on(async {
            let users = sqlx::query_as::<_, User>(
                "SELECT id, name, age, phone, address, birthday, since, introduction FROM user",
            )
                .fetch_all(&mut self.connection)
                .await
                .unwrap();

            let mut exam_query = "SELECT id, user_id, start_time, end_time, comment FROM examination WHERE user_id IN (".to_string();

            for i in 0..users.len() {
                exam_query += &format!("{}?", if i == 0 { "" } else { "," });
            }

            exam_query += ")";

            let exams = users.iter().fold(query(&exam_query), |q, u| {
                q.bind(u.id)
            }).fetch_all(&mut self.connection)
                .await
                .unwrap()
                .into_iter()
                .map(|row| Examination::from_row(&row).unwrap());

            let mut result = users.into_iter().map(|u| {
                (u.id, (u, Vec::new()))
            }).collect::<HashMap<_, _>>();

            for exam in exams {
                result.get_mut(&exam.user_id).unwrap().1.push(exam)
            }

            let _: Vec<(User, Vec<Examination>)> = result.into_iter().map(|(_, r)| r).collect();
        })
    }

    fn bench_associated_calc(&mut self) {
        self.runtime.block_on(async {
            let rows = sqlx::query(
                "SELECT \
                        u.id as id, u.name as name, u.age as age,\
                         u.phone as phone, u.address as address, u.birthday as birthday,\
                          u.since as since, u.introduction as introduction, \
                            (SELECT AVG(e.end_time - e.start_time) FROM examination e WHERE e.user_id = u.id) as a
                     FROM user u;",
            ).fetch_all(&mut self.connection)
                .await
                .unwrap();

            let _: Vec<(User, Option<Decimal>)> = rows.into_iter().map(|r| {
                let user = User::from_row(&r).unwrap();
                let avg: Option<Decimal> = r.get("a");
                (user, avg)
            }).collect();
        })
    }
}
