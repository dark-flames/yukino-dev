use sqlx::types::time::{Date, PrimitiveDateTime, Time};

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

    fn bench_insert(&mut self, users: Vec<Self::LocalNewUser>);

    fn bench_fetch_all(&mut self);
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
