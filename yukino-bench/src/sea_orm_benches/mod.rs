use sea_orm::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::Decimal;
use tokio::runtime::Runtime;

use examination::Entity as Examination;
use examination::Model as ExaminationModel;
use user::Entity as User;
use user::Model as UserModel;

use crate::CommonNewUser;
use crate::diesel_benches::{convert_date, convert_date_time};
use crate::Handler;

pub mod examination;
pub mod user;

pub struct SeaOrmHandler {
    connection: DatabaseConnection,
    runtime: Runtime,
}

impl Handler for SeaOrmHandler {
    type LocalNewUser = self::user::ActiveModel;

    fn orm_name() -> &'static str
    where
        Self: Sized,
    {
        "sea-orm"
    }

    fn create(url: &'static str) -> Self
    where
        Self: Sized,
    {
        let rt = Runtime::new().expect("Failed to start runtime");
        let db = rt.block_on(async {
            let pool = MySqlPoolOptions::new()
                .max_connections(1)
                .connect(url)
                .await
                .unwrap();
            let db = SqlxMySqlConnector::from_sqlx_mysql_pool(pool.clone());
            db
        });
        SeaOrmHandler {
            connection: db,
            runtime: rt,
        }
    }

    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>) {
        self.runtime.block_on(async {
            User::insert_many(users)
                .exec(&self.connection)
                .await
                .unwrap();
        })
    }

    fn bench_fetch_all(&mut self) {
        self.runtime.block_on(async {
            let _: Vec<UserModel> = User::find().all(&self.connection).await.unwrap();
        })
    }

    fn bench_zip_association(&mut self) {
        self.runtime.block_on(async {
            let _: Vec<(UserModel, Vec<ExaminationModel>)> = User::find()
                .find_with_related(Examination)
                .all(&self.connection)
                .await
                .unwrap();
        })
    }

    fn bench_associated_calc(&mut self) {
        self.runtime.block_on(async {
            let r: Vec<(UserModel, Vec<ExaminationModel>)> = User::find()
                .find_with_related(Examination)
                .all(&self.connection)
                .await
                .unwrap();

            let _: Vec<(UserModel, Option<Decimal>)> = r
                .into_iter()
                .map(|(user, exams)| {
                    (
                        user,
                        if exams.is_empty() {
                            None
                        } else {
                            let l = Decimal::from(exams.len());

                            Some(
                                Decimal::from(
                                    exams
                                        .into_iter()
                                        .map(|e| e.end_time - e.start_time)
                                        .sum::<i64>(),
                                ) / l,
                            )
                        },
                    )
                })
                .collect();
        });
    }
}

impl From<CommonNewUser> for self::user::ActiveModel {
    fn from(c: CommonNewUser) -> Self {
        self::user::ActiveModel {
            name: Set(c.name),
            age: Set(c.age),
            phone: Set(c.phone),
            address: Set(c.address),
            birthday: Set(convert_date(c.birthday)),
            since: Set(convert_date_time(c.since)),
            introduction: Set(c.introduction),
            ..Default::default()
        }
    }
}
