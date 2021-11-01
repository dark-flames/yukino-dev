use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct EntityViewPass();

impl EntityResolvePass for EntityViewPass {
    fn instance() -> Box<dyn EntityResolvePass>
        where
            Self: Sized,
    {
        Box::new(EntityViewPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::expr::{ComputationNode, Computation, Node, QueryResultNode};
            use yukino::query::SelectedItem;
            use yukino::interface::{EntityView};
            use yukino::view::View;
            use yukino::db::ty::ValuePack;
            use yukino::err::RuntimeResult;
            use yukino::expr::Expr;
            use yukino::interface::FieldMarker;
            use lazy_static::lazy_static;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}View", &entity.name);
        let entity_name = format_ident!("{}", &entity.name);
        let const_name = format_ident!("{}_VIEW", entity.name.to_snake_case().to_uppercase());

        let (fields, constructs): (Vec<_>, Vec<_>) = entity
            .fields
            .values()
            .map(|f| {
                let field_name = format_ident!("{}", f.path.field_name);
                let node_ty = &f.node_type;
                let node = &f.node;
                (
                    quote! {
                        pub #field_name: #node_ty
                    },
                    quote! {
                        #field_name: #node
                    },
                )
            })
            .unzip();
        let (selected_items, eval_items): (Vec<_>, Vec<_>) = entity
            .fields
            .values()
            .map(|f| {
                let field_name = format_ident!("{}", f.path.field_name);
                (
                    quote! {
                        result.extend(self.#field_name.collect_selected_items())
                    },
                    quote! {
                        #field_name: self.#field_name.eval(v)?
                    },
                )
            })
            .unzip();

        vec![quote! {
            #[derive(Clone)]
            pub struct #name {
                #(#fields),*
            }

            unsafe impl Sync for #name {}

            impl Node for #name {
                fn collect_selected_items(&self) -> Vec<SelectedItem> {
                    let mut result = vec![];
                    #(#selected_items;)*

                    result
                }
            }

            impl Computation for #name {
                type Output = #entity_name;

                fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output> {
                    Ok(#entity_name {
                        #(#eval_items),*
                    })
                }
            }

            lazy_static! {
                static ref #const_name: #name = #name {
                    #(#constructs),*
                }
            }

            impl View for #name {
                type Output = #entity_name;

                fn expr(&self) -> Expr<Self::Output> {
                    Expr::Computation(Box::new(#name::pure()))
                }
            }

            impl ComputationNode for #name {
                fn box_clone(&self) -> Box<dyn ComputationNode<Output=Self::Output>> {
                    Box::new(self.clone())
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;

                fn static_ref() -> &'static Self where Self: Sized {
                    &*#const_name
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
