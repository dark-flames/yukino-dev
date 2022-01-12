use sqlx::{Connection, MySqlConnection};
use tokio::runtime::Runtime;

use yukino::prelude::*;

use crate::interface::{CommonNewUser, Handler};

#[derive(Entity, Clone, Debug)]
pub struct Problem {
    #[id]
    #[auto_increment]
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Entity, Clone, Debug)]
#[name = "user"]
pub struct User {
    #[id]
    #[auto_increment]
    pub id: i32,
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: sqlx::types::time::Date,
    pub since: sqlx::types::time::PrimitiveDateTime,
    pub introduction: String,
}

impl From<CommonNewUser> for NewUser {
    fn from(c: CommonNewUser) -> Self {
        NewUser {
            name: c.name,
            age: c.age,
            phone: c.phone,
            address: c.address,
            birthday: c.birthday,
            since: c.since,
            introduction: c.introduction,
        }
    }
}

#[derive(Entity, Clone, Debug)]
pub struct Examination {
    #[id]
    #[auto_increment]
    pub id: i32,
    #[belongs_to(User)]
    pub user_id: i32,
    pub start_time: sqlx::types::time::PrimitiveDateTime,
    pub end_time: sqlx::types::time::PrimitiveDateTime,
}

#[derive(Entity, Clone, Debug)]
pub struct ExamProblem {
    #[id]
    #[auto_increment]
    pub id: i32,
    #[belongs_to(Problem)]
    pub problem_id: i32,
    #[belongs_to(Examination)]
    pub exam_id: i32,
    pub full_score: sqlx::types::Decimal,
}

#[derive(Entity, Clone, Debug)]
pub struct Answer {
    #[id]
    #[auto_increment]
    pub id: i32,
    pub content: String,
    #[belongs_to(ExamProblem)]
    pub exam_problem_id: i32,
    pub score: sqlx::types::Decimal,
}

pub struct YukinoHandler {
    connection: sqlx::MySqlConnection,
    runtime: Runtime,
}

impl Handler for YukinoHandler {
    type LocalNewUser = NewUser;

    fn orm_name() -> &'static str
    where
        Self: Sized,
    {
        "yukino"
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

        YukinoHandler {
            connection,
            runtime,
        }
    }

    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>) {
        self.runtime
            .block_on(async { users.insert_all().exec(&mut self.connection).await.unwrap() });
    }

    fn bench_fetch_all(&mut self) {
        self.runtime.block_on(async {
            User::all()
                .exec(&mut self.connection)
                .await
                .unwrap()
                .try_collect()
                .unwrap()
        });
    }
}
