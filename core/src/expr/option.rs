use crate::view::View;

#[allow(dead_code)]
pub struct Option<T: View> {
    inside: T,
}
