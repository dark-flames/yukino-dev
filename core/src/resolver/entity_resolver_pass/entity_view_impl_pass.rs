use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use interface::DefinitionType;

use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};

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
            use yukino::query::{SortHelper, SortResult};
            use yukino::view::{SingleExprView, ExprViewBox, ExprViewBoxWithTag, ExprView, EntityView, EntityViewTag, TagList1, TagsOfValueView, AnyTagExprView, VerticalExprView, VerticalView, TagList, EntityVerticalView};
            use yukino::query_builder::{Expr, Alias, DatabaseValue, OrderByItem};
            use yukino::err::{RuntimeResult, YukinoError};
        }]
    }

    fn get_entity_implements(&mut self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = &entity.view_name;
        let vertical_name = &entity.vertical_view_name;
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
            vertical_fields,
            to_vertical_branches,
        ) = iter
            .enumerate()
            .fold(
                (vec![], vec![], quote! {arr![Expr;]}, vec![], vec![], vec![], vec![], vec![], vec![]),
                |(mut fields, mut tmp, rst, mut expr_tmp, mut expr_branch, mut clone, mut pure, mut vertical_fields, mut vertical_branches), (index, f)| {
                    let field_name = format_ident!("{}", f.path.field_name);
                    let field_value_count = format_ident!("U{}", f.converter_param_count);
                    let view_path = &f.view_path;
                    let vertical_view_path = &f.vertical_view_path;
                    let view_ty = &f.view_ty;
                    let vertical_view_ty = &f.vertical_view_ty;
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
                            Concat::concat(#rst, #field_name)
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

            impl ExprView<#entity_name> for #name {
                type Tags = TagList1<EntityViewTag>;

                fn from_exprs(exprs: GenericArray<Expr, ValueCountOf<#entity_name>>)
                -> ExprViewBox<#entity_name>
                where
                    Self: Sized {
                    let rest = exprs;
                    #(#from_expr_tmp;)*

                    Box::new(#name {
                        #(#from_expr_branches),*
                    })
                }

                fn expr_clone(&self) -> ExprViewBoxWithTag<#entity_name, Self::Tags>
                where
                    Self: Sized {
                    Box::new(#name {
                        #(#clone_branches),*
                    })
                }

                fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<#entity_name>> {
                    #(#collect_tmp;)*

                    #collect_rst
                }

                fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<#entity_name>>) -> RuntimeResult<#entity_name> {
                    (*#entity_name::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
                }
            }

            impl EntityView for #name {
                type Entity = #entity_name;

                fn pure(alias: &Alias) -> Self where Self: Sized {
                    #name {
                        #(#pure_branches),*
                    }
                }

                fn vertical(self) -> <Self::Entity as EntityWithView>::VerticalView where Self: Sized {
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
                _order_by: Vec<OrderByItem>
            }

            impl VerticalView<#entity_name> for #vertical_name {
                type RowView = #name;

                fn map<
                    R: Value,
                    RTags: TagList,
                    RV: Into<ExprViewBoxWithTag<R, RTags>>,
                    F: Fn(Self::RowView) -> RV,
                >(
                    self,
                    f: F,
                ) -> VerticalExprView<R, RTags> {
                    VerticalExprView::create(
                        f(self._row_view).into(),
                        self._order_by
                    )
                }

                fn sort<R: SortResult, F: Fn(Self::RowView, SortHelper) -> R>(mut self, f: F) -> Self {
                    let result = f(self._row_view.clone(), SortHelper::create());
                    self._order_by = result.order_by_items();
                    self
                }
            }

            impl EntityVerticalView for #vertical_name {
                type Entity = #entity_name;
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
