use crate::interface::def::DefinitionType;
use crate::schema::entity::{EntityResolvePass, ResolvedEntity};
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
            use yukino::view::{View, ViewNode, ExprView};
            use yukino::query::{Expr, Alias};
            use yukino::interface::EntityView;
            use yukino::db::ty::DatabaseValue;
            use yukino::err::{RuntimeResult, YukinoError};
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
                let field_ty = &f.view_ty;
                let view = &f.view;
                (
                    quote! {
                        pub #field_name: #field_ty
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
                        #field_name: self.#field_name.clone()
                    },
                    quote! {
                        exprs.extend(self.#field_name.collect_expr())
                    },
                )
            })
            .unzip();

        vec![quote! {
            #[derive(Debug)]
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

            impl View<#entity_name> for #name {
                fn view_node(&self) -> ViewNode<#entity_name> {
                    ViewNode::Expr(ExprView::create(self.collect_expr()))
                }

                fn collect_expr(&self) -> Vec<Expr> {
                    let mut exprs = vec![];

                    #(#node_items;)*

                    exprs
                }

                fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<#entity_name> {
                    (*#entity_name::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
                }

                fn clone(&self) -> Self {
                    Clone::clone(self)
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;

                fn pure(alias: &Alias) -> Self where Self: Sized {
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
