use yukino::Entity;

#[derive(Entity, Clone, Debug)]
pub struct Basic {
    id: u32,
    boolean: bool,
    u_short: u16,
    short: i16,
    u_int: u32,
    int: i32,
    u_long: u64,
    long: i64,
    float: f32,
    double: f64,
    string: String,
    character: char,
    optional: Option<u32>,
}