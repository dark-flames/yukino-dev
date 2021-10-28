use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use unzip3::Unzip3;

pub struct EntityViewImplementPass();

impl EntityResolvePass for EntityViewImplementPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(EntityViewImplementPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::view::*;
            use yukino::interface::EntityView;
            use yukino::query::computation::Computation;
            use yukino::query::optimizer::{QueryOptimizer, SelectAppendOptimizer};
            use yukino::interface::{FieldMarker, FieldView};
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}View", entity.name);
        let entity_name = format_ident!("{}", entity.name);
        let marker_mod = format_ident!("{}", entity.name.to_snake_case());
        let (fields, construct_fields, computations): (Vec<_>, Vec<_>, Vec<_>) = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let name = format_ident!("{}", f.path.field_name);
                let ty = &f.view_type;
                let marker_name = format_ident!("{}", f.path.field_name.to_snake_case());
                let view_ty = &f.view_type;
                (
                    quote! {
                        pub #name: #ty
                    },
                    quote! {
                        #name: #view_ty::create(
                            #marker_mod::#marker_name::data_converter()
                        )
                    },
                    quote! {
                        #name: {
                            (*#marker_mod::#marker_name::data_converter().field_value_converter())(v)?
                        }
                    }
                )
            })
            .unzip3();

        let append_optimizer: Vec<_> = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let marker_name = format_ident!("{}", f.path.field_name.to_snake_case());

                quote! {
                    .append::<#marker_mod::#marker_name>()
                }
            })
            .collect();

        vec![quote! {
            pub struct #name {
                #(#fields),*
            }

            impl View for #name {
                type Output = #entity_name;
                fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                    Computation::create(Box::new(
                        |v| {
                            Ok(#entity_name {
                                #(#computations),*
                            })
                        }
                    ))
                }

                fn optimizer(&self) -> Box<dyn QueryOptimizer> {
                    let mut optimizer: SelectAppendOptimizer = Default::default();
                    optimizer
                        #(#append_optimizer)*;

                    Box::new(optimizer)
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;
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
