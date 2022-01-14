use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::types::Decimal;
use sqlx::types::time::{Date, PrimitiveDateTime, Time};

use crate::{CommonNewUser, Handler};
use crate::diesel::*;

table! {
    examination (id) {
        id -> Integer,
        user_id -> Integer,
        start_time -> Bigint,
        end_time -> Bigint,
        comment -> Text,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Varchar,
        age -> Unsigned<Smallint>,
        phone -> Varchar,
        address -> Varchar,
        birthday -> Date,
        since -> Datetime,
        introduction -> Text,
    }
}

joinable!(examination -> user (user_id));

allow_tables_to_appear_in_same_query!(examination, user,);

pub struct Problem {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName,
)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: NaiveDate,
    pub since: NaiveDateTime,
    pub introduction: String,
}

#[derive(Debug, PartialEq, Eq, Queryable, Clone, Insertable, AsChangeset)]
#[table_name = "user"]
#[diesel(treat_none_as_default_value = false)]
pub struct NewUser {
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: NaiveDate,
    pub since: NaiveDateTime,
    pub introduction: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Associations, QueryableByName)]
#[belongs_to(User)]
#[table_name = "examination"]
pub struct Examination {
    pub id: i32,
    pub user_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub comment: String,
}

pub struct ExamProblem {
    pub id: i32,
    pub problem_id: i32,
    pub exam_id: i32,
    pub full_score: sqlx::types::Decimal,
}

pub struct Answer {
    pub id: i32,
    pub content: String,
    pub exam_problem_id: i32,
    pub score: sqlx::types::Decimal,
}

pub struct DieselHandler {
    connection: MysqlConnection,
}

impl Handler for DieselHandler {
    type LocalNewUser = NewUser;

    fn orm_name() -> &'static str
    where
        Self: Sized,
    {
        "diesel"
    }

    fn create(url: &'static str) -> Self
    where
        Self: Sized,
    {
        DieselHandler {
            connection: MysqlConnection::establish(url).expect("fail to create connection"),
        }
    }

    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>) {
        insert_into(user::table)
            .values(users)
            .execute(&self.connection)
            .unwrap();
    }

    fn bench_fetch_all(&mut self) {
        user::table.load::<User>(&self.connection).unwrap();
    }

    fn bench_zip_association(&mut self) {
        let users = user::table.load::<User>(&self.connection).unwrap();
        let examinations = Examination::belonging_to(&users)
            .load::<Examination>(&self.connection)
            .unwrap()
            .grouped_by(&users);

        let _: Vec<(User, Vec<Examination>)> = users.into_iter().zip(examinations).collect();
    }

    fn bench_associated_calc(&mut self) {
        let users = user::table.load::<User>(&self.connection).unwrap();
        let examinations = Examination::belonging_to(&users)
            .load::<Examination>(&self.connection)
            .unwrap()
            .grouped_by(&users);

        let _: Vec<(User, Option<Decimal>)> = users
            .into_iter()
            .zip(examinations)
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
    }
}

pub fn convert_date(sqlx_date: Date) -> NaiveDate {
    let (y, m, d) = sqlx_date.as_ymd();
    NaiveDate::from_ymd(y, m as u32, d as u32)
}

pub fn convert_time(sqlx_time: Time) -> NaiveTime {
    NaiveTime::from_hms(
        sqlx_time.clone().hour() as u32,
        sqlx_time.clone().minute() as u32,
        sqlx_time.second() as u32,
    )
}

pub fn convert_date_time(sqlx_date_time: PrimitiveDateTime) -> NaiveDateTime {
    NaiveDateTime::new(
        convert_date(sqlx_date_time.clone().date()),
        convert_time(sqlx_date_time.time()),
    )
}

impl From<CommonNewUser> for NewUser {
    fn from(c: CommonNewUser) -> Self {
        NewUser {
            name: c.name,
            age: c.age,
            phone: c.phone,
            address: c.address,
            birthday: convert_date(c.birthday),
            since: convert_date_time(c.since),
            introduction: c.introduction,
        }
    }
}
