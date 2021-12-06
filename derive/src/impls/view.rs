use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct ViewImplementor;

impl Implementor for ViewImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let name = &resolved.view_name;
        let vertical_name = &resolved.vertical_name;
        let entity_name = &resolved.entity_name;

        let last_index = resolved.fields.len() - 1;

        let (
            view_fields,
            collect_tmp,
            collect_rst,
            from_expr_tmp,
            from_expr_branches,
            clone_branches,
            pure_branches,
            vertical_fields,
            to_vertical_branches,
        ) = resolved.fields.iter()
            .enumerate()
            .fold(
                (vec![], vec![], quote! {yukino::generic_array::arr![yukino::query_builder::Expr;]}, vec![], vec![], vec![], vec![], vec![], vec![]),
                |(mut fields, mut tmp, rst, mut expr_tmp, mut expr_branch, mut clone, mut pure, mut vertical_fields, mut vertical_branches), (index, f)| {
                    let field_name = &f.name;
                    let field_value_count = {
                        let type_num = format_ident!("U{}", f.converter_value_count);
                        quote! {
                            yukino::generic_array::typenum::#type_num
                        }
                    };
                    let view_path = &f.view_full_path;
                    let vertical_view_path = &f.vertical_full_path;
                    let view_ty = &f.view_ty;
                    let vertical_view_ty = &f.vertical_ty;
                    let view = &f.view_construct;
                    fields.push(quote! {
                        pub #field_name: #view_ty
                    });
                    tmp.push(quote! {
                        let #field_name = self.#field_name.collect_expr()
                    });

                    if index == last_index {
                        expr_tmp.push(quote! {
                            let (#field_name, _) = yukino::generic_array::sequence::Split::<
                                _, #field_value_count
                            >::split(rest)
                        });
                    } else {
                        expr_tmp.push(quote! {
                            let (#field_name, rest) =  yukino::generic_array::sequence::Split::<
                                _, #field_value_count
                            >::split(rest)
                        });
                    }

                    expr_branch.push(quote! {
                        #field_name: #view_path::from_exprs(#field_name)
                    });

                    clone.push(quote! {
                        #field_name: self.#field_name.clone()
                    });

                    pure.push(quote! {
                        #field_name: #view
                    });

                    vertical_fields.push(quote! {
                        pub #field_name: #vertical_view_ty
                    });

                    vertical_branches.push(quote! {
                        #field_name: #vertical_view_path::create(self.#field_name, vec![])
                    });


                    (
                        fields,
                        tmp,
                        quote! {
                             yukino::generic_array::sequence::Concat::concat(#rst, #field_name)
                        },
                        expr_tmp,
                        expr_branch,
                        clone,
                        pure,
                        vertical_fields,
                        vertical_branches
                    )
                }
            );

        vec![quote! {
            #[derive(Clone)]
            pub struct #name {
                #(#view_fields),*
            }

            impl yukino::view::ExprView<#entity_name> for #name {
                type Tags = yukino::view::TagList1<yukino::view::EntityViewTag>;

                fn from_exprs(
                    exprs: yukino::generic_array::GenericArray<
                        yukino::query_builder::Expr, yukino::view::ValueCountOf<#entity_name>
                    >
                )-> yukino::view::ExprViewBox<#entity_name>
                where
                    Self: Sized {
                    use yukino::view::ExprView;
                    let rest = exprs;
                    #(#from_expr_tmp;)*

                    Box::new(#name {
                        #(#from_expr_branches),*
                    })
                }

                fn expr_clone(&self) -> yukino::view::ExprViewBoxWithTag<#entity_name, Self::Tags>
                where
                    Self: Sized {
                    Box::new(#name {
                        #(#clone_branches),*
                    })
                }

                fn collect_expr(&self) -> yukino::generic_array::GenericArray<
                    yukino::query_builder::Expr, yukino::view::ValueCountOf<#entity_name>
                > {
                    #(#collect_tmp;)*

                    #collect_rst
                }

                fn eval(
                    &self,
                    v: &yukino::generic_array::GenericArray<
                        yukino::query_builder::DatabaseValue,
                        yukino::view::ValueCountOf<#entity_name>>
                ) -> yukino::err::RuntimeResult<#entity_name> {
                    use yukino::view::Value;
                    use yukino::err::YukinoError;
                    (*#entity_name::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
                }
            }

            impl yukino::view::EntityView for #name {
                type Entity = #entity_name;

                fn pure(alias: &yukino::query_builder::Alias) -> Self where Self: Sized {
                    #name {
                        #(#pure_branches),*
                    }
                }

                fn vertical(self) -> <Self::Entity as yukino::view::EntityWithView>::VerticalView where Self: Sized {
                    let _row_view = self.clone();
                    #vertical_name {
                        #(#to_vertical_branches,)*
                        _row_view,
                        _order_by: vec![],
                    }
                }
            }

            pub struct #vertical_name {
                #(#vertical_fields,)*
                _row_view: #name,
                _order_by: Vec<yukino::query_builder::OrderByItem>
            }

            impl yukino::view::VerticalView<#entity_name> for #vertical_name {
                type RowView = #name;

                fn row_view(&self) -> Self::RowView {
                    self._row_view.clone()
                }

                fn map<
                    R: yukino::view::Value,
                    RTags: yukino::view::TagList,
                    RV: Into<yukino::view::ExprViewBoxWithTag<R, RTags>>,
                    F: Fn(Self::RowView) -> RV,
                >(
                    self,
                    f: F,
                ) -> yukino::view::VerticalExprView<R, RTags> {
                    yukino::view::VerticalExprView::create(
                        f(self._row_view).into(),
                        self._order_by
                    )
                }

                fn sort<R: yukino::query::SortResult, F: Fn(Self::RowView, yukino::query::SortHelper) -> R>(mut self, f: F) -> Self {
                    use yukino::query::SortResult;
                    let result = f(self.row_view(), yukino::query::SortHelper::create());
                    self._order_by = result.order_by_items();
                    self
                }
            }

            impl yukino::view::EntityVerticalView for #vertical_name {
                type Entity = #entity_name;
            }
        }]
    }
}