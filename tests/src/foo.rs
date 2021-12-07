use yukino::Entity;

#[derive(Entity, Clone, Debug)]
pub struct Foo {
    #[id]
    pub id: u32,
    pub boolean: bool,
    pub u_short: u16,
    pub short: i16,
    pub u_int: u32,
    pub int: i32,
    pub u_long: u64,
    pub long: i64,
    pub float: f32,
    pub double: f64,
    pub string: String,
    pub character: char,
    pub optional: Option<u32>,
}