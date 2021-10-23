use proc_macro2::TokenStream;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};

pub struct EntityViewImplementPass {}

impl EntityResolvePass for EntityViewImplementPass {
    fn get_entity_implements(&self, _entity: &ResolvedEntity) -> Vec<TokenStream> {
        todo!("impl Entity for entity")
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        todo!()
    }
}