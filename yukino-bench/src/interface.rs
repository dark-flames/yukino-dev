use rand::Rng;
use sqlx::{Connection, MySqlConnection};
use sqlx::types::time::{Date, PrimitiveDateTime, Time};
use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
pub struct CommonNewUser {
    pub name: String,
    pub age: u16,
    pub phone: String,
    pub address: String,
    pub birthday: sqlx::types::time::Date,
    pub since: sqlx::types::time::PrimitiveDateTime,
    pub introduction: String,
}

pub trait Handler: 'static {
    type LocalNewUser: From<CommonNewUser> + 'static;

    fn convert_users(users: Vec<CommonNewUser>) -> Vec<Self::LocalNewUser> {
        users.into_iter().map(Into::into).collect()
    }

    fn orm_name() -> &'static str
    where
        Self: Sized;

    fn create(url: &'static str) -> Self
    where
        Self: Sized;

    // Insert users into db
    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>);

    // Fetch all users from db
    fn bench_fetch_all(&mut self);

    fn bench_zip_association(&mut self);

    // Calculate the average exam time per user as Vec<(User, Decimal)>
    fn bench_associated_calc(&mut self);
}

pub fn generate_user(size: usize, introduction_size: usize) -> Vec<CommonNewUser> {
    let birthday = Date::try_from_ymd(1919, 8, 10).unwrap();
    let since = PrimitiveDateTime::new(birthday, Time::try_from_hms(11, 45, 51).unwrap());
    let introduction = "t".repeat(introduction_size);

    (0..size)
        .map(|idx| CommonNewUser {
            name: format!("User {}", idx),
            age: 24,
            phone: "1145141919810".to_string(),
            address: "Shimokitazawa, Tokyo, Japan".to_string(),
            birthday,
            since,
            introduction: introduction.clone(),
        })
        .collect()
}

pub fn generate_data(
    url: &'static str,
    user_size: usize,
    exam_per_user: usize,
    comment_size: usize,
) {
    use crate::yukino_benches::*;
    use yukino::prelude::*;
    let mut rnd = rand::thread_rng();
    let birthday = Date::try_from_ymd(1919, 8, 10).unwrap();
    let since = PrimitiveDateTime::new(birthday, Time::try_from_hms(11, 45, 51).unwrap());
    let (users, examinations): (Vec<User>, Vec<Vec<NewExamination>>) = (1..user_size + 1)
        .into_iter()
        .map(|uid| {
            (
                User {
                    id: uid as i32,
                    name: format!("User {}", uid),
                    age: 24,
                    phone: "1145141919810".to_string(),
                    address: "Shimokitazawa, Tokyo, Japan".to_string(),
                    birthday,
                    since,
                    introduction: "a".repeat(100),
                },
                (0..exam_per_user)
                    .into_iter()
                    .map(|_| {
                        let start_time = rnd.gen_range(100..100000);
                        NewExamination {
                            user_id: uid as i32,
                            start_time,
                            end_time: rnd.gen_range((start_time + 1)..(start_time * 2)),
                            comment: "a".repeat(comment_size),
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unzip();

    let examinations: Vec<NewExamination> = examinations.into_iter().flatten().collect();
    Runtime::new().unwrap().block_on(async {
        let mut conn = MySqlConnection::connect(url).await.unwrap();

        sqlx::query("SET FOREIGN_KEY_CHECKS = 0;")
            .execute(&mut conn)
            .await
            .unwrap();

        users.insert_all().exec(&mut conn).await.unwrap();
        for e in examinations {
            e.insert().exec(&mut conn).await.unwrap();
        }

        sqlx::query("SET FOREIGN_KEY_CHECKS = 1;")
            .execute(&mut conn)
            .await
            .unwrap();
    })
}
