use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use interface::DefinitionType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Type};

pub struct EntityStructPass();

impl EntityResolvePass for EntityStructPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(EntityStructPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![]
    }

    fn get_entity_implements(&mut self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}", entity.name);
        let fields = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let name = format_ident!("{}", f.definition.name);
                let ty: Type = parse_str(f.definition.ty.as_str()).unwrap();
                quote! {
                    pub #name: #ty
                }
            })
            .collect::<Vec<_>>();

        vec![quote! {
            #[derive(Clone, Debug)]
            pub struct #name {
                #(#fields),*
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
