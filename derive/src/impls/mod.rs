use proc_macro2::TokenStream;

pub use association::*;
pub use converter::*;
pub use entity::*;
pub use marker::*;
pub use primary::*;
pub use view::*;

use crate::resolved::ResolvedEntity;

mod converter;
mod entity;
mod view;
mod primary;
mod association;
mod marker;

pub trait Implementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream>;
}