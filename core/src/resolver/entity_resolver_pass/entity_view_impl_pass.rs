use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use interface::DefinitionType;
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
            use yukino::view::{SingleExprView, ViewBox, ExprViewBox, ExprView, EntityView};
            use yukino::query_builder::{Expr, Alias, DatabaseValue, ExprNode, ExprVisitor, ExprMutVisitor};
            use yukino::err::{RuntimeResult, YukinoError};
        }]
    }

    fn get_entity_implements(&mut self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}View", &entity.name);
        let entity_name = format_ident!("{}", &entity.name);
        let iter = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated);

        let last_index = iter.clone().count() - 1;

        let (
            view_fields,
            collect_tmp,
            collect_rst,
            from_expr_tmp,
            from_expr_branches,
            clone_branches,
            pure_branches,
            apply_branches,
            apply_mut_branches
        ) = iter
            .enumerate()
            .fold(
                (vec![], vec![], quote! {arr![Expr;]}, vec![], vec![], vec![], vec![], vec![], vec![]),
                |(mut fields, mut tmp, rst, mut expr_tmp, mut expr_branch, mut clone, mut pure, mut apply, mut apply_mut), (index, f)| {
                    let field_name = format_ident!("{}", f.path.field_name);
                    let field_value_count = format_ident!("U{}", f.converter_param_count);
                    let view_path = &f.view_path;
                    let view_ty = &f.view_ty;
                    let view = &f.view;
                    fields.push(quote! {
                        pub #field_name: #view_ty
                    });
                    tmp.push(quote! {
                        let #field_name = self.#field_name.collect_expr()
                    });

                    if index == last_index {
                        expr_tmp.push(quote! {
                            let (#field_name, _) = Split::<_, typenum::#field_value_count>::split(rest)
                        });
                    } else {
                        expr_tmp.push(quote! {
                            let (#field_name, rest) = Split::<_, typenum::#field_value_count>::split(rest)
                        });
                    }

                    expr_branch.push(quote! {
                        #field_name: Box::new(#view_path::from_exprs(#field_name))
                    });

                    clone.push(quote! {
                        #field_name: self.#field_name.clone()
                    });

                    pure.push(quote! {
                        #field_name: #view
                    });

                    apply.push(quote! {
                        self.#field_name.apply(visitor);
                    });

                    apply_mut.push(quote! {
                        self.#field_name.apply_mut(visitor);
                    });


                    (
                        fields,
                        tmp,
                        quote! {
                            Concat::concat(#rst, #field_name)
                        },
                        expr_tmp,
                        expr_branch,
                        clone,
                        pure,
                        apply,
                        apply_mut
                    )
                }
            );

        vec![quote! {
            #[derive(Clone)]
            pub struct #name {
                #(#view_fields),*
            }

            impl ExprNode for #name {
                fn apply(&self, visitor: &mut dyn ExprVisitor) {
                    #(#apply_branches;)*;
                }

                fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
                    #(#apply_mut_branches;)*;
                }
            }

            impl View<#entity_name, ValueCountOf<#entity_name>> for #name {
                fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<#entity_name>> {
                    #(#collect_tmp;)*

                    #collect_rst
                }

                fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<#entity_name>>) -> RuntimeResult<#entity_name> {
                    (*#entity_name::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
                }

                fn view_clone(&self) -> ViewBox<#entity_name, ValueCountOf<#entity_name>> {
                    Box::new(self.clone())
                }
            }

            impl ExprView<#entity_name> for #name {

                fn from_exprs(exprs: GenericArray<Expr, ValueCountOf<#entity_name>>) -> Self
                where
                    Self: Sized {
                    let rest = exprs;
                    #(#from_expr_tmp;)*

                    #name {
                        #(#from_expr_branches),*
                    }
                }

                fn expr_clone(&self) -> ExprViewBox<#entity_name>
                where
                    Self: Sized {
                    Box::new(#name {
                        #(#clone_branches),*
                    })
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;

                fn pure(alias: &Alias) -> Self where Self: Sized {
                    #name {
                        #(#pure_branches),*
                    }
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
