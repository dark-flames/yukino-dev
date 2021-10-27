#[Entity()]
pub struct Numeric {
    #[ID]
    id: u32,
    u_short: u16,
    short: i16,
    u_int: u32,
    int: i32,
    u_long: u64,
    long: i64,
    float: f32,
    double: f64,
}