use sqlx::{Connection, MySqlConnection};
use sqlx::types::Decimal;
use tokio::runtime::Runtime;

use yukino::prelude::*;

use crate::interface::{CommonNewUser, Handler};

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

#[derive(Entity, Clone, Debug)]
pub struct Examination {
    #[id]
    #[auto_increment]
    pub id: i32,
    #[belongs_to(User)]
    pub user_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub comment: String,
}

pub struct YukinoHandler {
    connection: sqlx::MySqlConnection,
    runtime: Runtime,
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

    fn bench_zip_association(&mut self) {
        self.runtime.block_on(async {
            let users = User::all()
                .exec(&mut self.connection)
                .await
                .unwrap()
                .try_collect()
                .unwrap();
            let entities = Examination::belonging_to(&users)
                .exec(&mut self.connection)
                .await
                .unwrap()
                .try_collect()
                .unwrap();

            let _: Vec<(User, Vec<Examination>)> = users.join(entities);
        })
    }

    fn bench_associated_calc(&mut self) {
        self.runtime.block_on(async {
            let _: Vec<(User, Option<Decimal>)> = User::all()
                .map(|u| {
                    let examinations = Examination::belonging_to_view(&u);
                    (
                        u.into_expr(),
                        examinations
                            .fold(|e_v| e_v.map(|e| e.end_time - e.start_time).average())
                            .into_expr(),
                    )
                })
                .exec(&mut self.connection)
                .await
                .unwrap()
                .try_collect()
                .unwrap();
        })
    }
}
