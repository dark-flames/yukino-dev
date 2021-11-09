use crate::schema::entity::{EntityResolvePass, ResolvedEntity};
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
            use yukino::view::Value;
            use yukino::converter::ConverterRef;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}", &entity.name);
        let view_name = &entity.view_name;
        let converter_name = &entity.converter_name;
        vec![quote! {
            impl Entity for #name {
                type View = #view_name;
            }

            impl Value for #name {
                fn converter() -> ConverterRef<Self> where Self: Sized {
                    #converter_name::instance()
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
