use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct EntityImplementPass();

impl EntityResolvePass for EntityImplementPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(EntityImplementPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::interface::Entity;
            use yukino::expr::Value;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}", &entity.name);
        let view_name = &entity.view_name;
        vec![quote! {
            impl Entity for #name {
                type View = #view_name;
            }

            impl Value for #name {}
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
