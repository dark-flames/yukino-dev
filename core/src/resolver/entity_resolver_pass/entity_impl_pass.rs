use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use proc_macro2::TokenStream;

pub struct EntityImplementPass {}

impl EntityResolvePass for EntityImplementPass {
    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![]
    }

    fn get_entity_implements(&self, _entity: &ResolvedEntity) -> Vec<TokenStream> {
        vec![]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
