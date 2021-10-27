use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct EntityViewImplementPass {}

impl EntityResolvePass for EntityViewImplementPass {
    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::view::*;
            use yukino::interface::EntityView;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}View", entity.name);
        let entity_name = format_ident!("{}", entity.name);
        let (fields, construct_fields) = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let name = format_ident!("{}", f.definition.name);
                let ty = &f.view_type;
                let converter = &f.converter;
                (
                    quote! {
                        pub #name: #ty
                    },
                    quote! {
                        #name: #converter
                    }
                )
            }).unzip::<_, _, Vec<_>, Vec<_>>();

        vec![quote! {
            pub struct #name {
                #(#fields),*
            }

            impl View for #name {
                type Output = #entity_name;
                fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                    todo!()
                }

                fn optimizer(&self) -> Box<dyn QueryOptimizer> {
                    todo!()
                }
            }

            impl EntityView for #name {
                type Entity = $entity_name;
                fn pure() -> Self where Self: Sized {
                    #name {
                        #(#construct_fields),*
                    }
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
