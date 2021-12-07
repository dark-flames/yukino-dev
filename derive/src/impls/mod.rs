use proc_macro2::TokenStream;

pub use converter::*;
pub use entity::*;
pub use primary::*;
pub use view::*;

use crate::resolved::ResolvedEntity;

mod converter;
mod entity;
mod view;
mod primary;

pub trait Implementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream>;
}