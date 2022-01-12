use sqlx::{Connection, MySqlConnection};
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
            sqlx::query_as::<_, User>("SELECT id, name, hair_color FROM user")
                .fetch_all(&mut self.connection)
                .await
                .unwrap();
        })
    }
}
