use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
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
            use yukino::view::{ViewBox, View, Computation, ViewNode, ExprView};
            use yukino::query::{Expr, Alias};
            use yukino::interface::EntityView;
            use yukino::db::ty::DatabaseValue;
            use yukino::err::RuntimeResult;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}View", &entity.name);
        let entity_name = format_ident!("{}", &entity.name);
        let (fields, constructs): (Vec<_>, Vec<_>) = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|f| {
                let field_name = format_ident!("{}", f.path.field_name);
                let field_ty = &f.ty;
                let view = &f.view;
                (
                    quote! {
                        pub #field_name: ViewBox<#field_ty>
                    },
                    quote! {
                        #field_name: #view
                    },
                )
            })
            .unzip();
        let (clone_branches, node_items): (Vec<_>, Vec<_>) = entity
            .fields
            .iter()
            .map(|f| {
                let field_name = format_ident!("{}", f.path.field_name);
                (
                    quote! {
                        #field_name: self.#field_name.box_clone()
                    },
                    quote! {
                        exprs.extend(self.#field_name.collect_expr())
                    },
                )
            })
            .unzip();

        vec![quote! {
            pub struct #name {
                #(#fields),*
            }

            unsafe impl Sync for #name {}

            impl Clone for #name {
                fn clone(&self) -> Self {
                    #name {
                        #(#clone_branches),*
                    }
                }
            }

            impl Computation for #name {
                type Output = #entity_name;

                fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
                    (*#entity_name::converter().deserializer())(v)
                }
            }

            impl View<#entity_name> for #name {
                fn view_node(&self) -> ViewNode<#entity_name> {
                    ViewNode::Expr(ExprView::create(self.collect_expr()))
                }

                fn collect_expr(&self) -> Vec<Expr> {
                    let mut exprs = vec![];

                    #(#node_items;)*

                    exprs
                }

                fn box_clone(&self) -> ViewBox<#entity_name> {
                    Box::new(self.clone())
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;

                fn pure(alias: Alias) -> Self where Self: Sized {
                    #name {
                        #(#constructs),*
                    }
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
