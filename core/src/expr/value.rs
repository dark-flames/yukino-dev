pub trait Value: 'static + Clone {}

pub trait CopyValue: Value + Copy {}

macro_rules! impl_value {
    ($ty: ty) => {
        impl Value for $ty {}
    };
    (copy $ty: ty) => {
        impl Value for $ty {}
        impl CopyValue for $ty {}
    };
}

impl_value!(copy u8);
impl_value!(copy u16);
impl_value!(copy u32);
impl_value!(copy u64);
impl_value!(copy u128);
impl_value!(copy i8);
impl_value!(copy i16);
impl_value!(copy i32);
impl_value!(copy i64);
impl_value!(copy i128);
impl_value!(copy f32);
impl_value!(copy f64);
impl_value!(copy usize);
impl_value!(copy isize);
impl_value!(copy char);
impl_value!(String);

impl<T: Value> Value for Option<T> {}

impl<T: CopyValue> CopyValue for Option<T> {}
