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
            use yukino::query::optimizer::{SelectAppendOptimizer, OptimizerBox};
            use yukino::interface::{FieldMarker, FieldView};
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let view_name = format_ident!("{}View", entity.name);
        let entity_name = format_ident!("{}", entity.name);
        let marker_mod = format_ident!("{}", entity.name.to_snake_case());
        let (fields, computation_tmp, computations): (Vec<_>, Vec<_>, Vec<_>) = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let name = format_ident!("{}", f.path.field_name);
                let tmp_name = format_ident!("{}_computation", f.path.field_name);
                let ty = &f.view_type;
                (
                    quote! {
                        pub #name: Box<#ty>
                    },
                    quote! {
                        let #tmp_name = self.#name.computation()
                    },
                    quote! {
                        #name: #tmp_name.eval(v)?
                    },
                )
            })
            .unzip3();

        let (append_optimizer, construct_fields): (Vec<_>, Vec<_>) = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let name = format_ident!("{}", f.path.field_name);
                let marker_name = format_ident!("{}", f.path.field_name.to_snake_case());
                let view_ty = &f.view_type;
                (
                    quote! {
                        .append::<#marker_mod::#marker_name>()
                    },
                    quote! {
                        #name: #view_ty::create(
                            #marker_mod::#marker_name::data_converter()
                        )
                    },
                )
            })
            .unzip();

        vec![quote! {
            pub struct #view_name {
                #(#fields),*
            }

            impl View for #view_name {
                type Output = #entity_name;
                fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                    #(#computation_tmp;)*
                    Computation::create(Box::new(move |v| {
                        Ok(#entity_name {
                            #(#computations),*
                        })
                    }))
                }

                fn optimizer(&self) -> OptimizerBox {
                    let mut optimizer = SelectAppendOptimizer::create();
                    optimizer
                        #(#append_optimizer)*;

                    optimizer
                }
            }

            impl EntityView for #view_name {
                type Entity = #entity_name;
                fn pure() -> Self where Self: Sized {
                    #view_name {
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
