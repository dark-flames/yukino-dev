use proc_macro2::TokenStream;

pub use association::*;
pub use converter::*;
pub use entity::*;
pub use insert::*;
pub use marker::*;
pub use primary::*;
pub use view::*;

use crate::resolved::ResolvedEntity;

mod association;
mod converter;
mod entity;
mod insert;
mod marker;
mod primary;
mod view;

pub trait Implementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream>;
}
