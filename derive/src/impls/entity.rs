use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct EntityImplementor;

impl Implementor for EntityImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let name = &resolved.entity_name;
        let view_name = &resolved.view_name;
        let vertical_view_name = &resolved.vertical_name;
        let converter_name = &resolved.converter_name;
        let table_name = &resolved.table_name;
        let value_count = {
            let type_num = format_ident!(
                "U{}",
                resolved
                    .fields
                    .iter()
                    .fold(0, |c, i| c + i.converter_value_count)
            );

            quote! {
                yukino::generic_array::typenum::#type_num
            }
        };
        let insert_branches: Vec<_> = resolved
            .fields
            .iter()
            .map(|field| {
                let field_name = &field.name;
                let tmp = format_ident!("{}_tmp", field.name.to_string().to_snake_case());
                let column_branches: Vec<_> = field
                    .definition
                    .columns
                    .iter()
                    .map(|c| {
                        let column_name = &c.name;
                        quote! {
                            .set(
                                #column_name.to_string(),
                                yukino::query_builder::AssignmentValue::Expr(
                                    yukino::query_builder::Expr::Lit(#tmp.next().unwrap())
                                )
                            )
                        }
                    })
                    .collect();

                quote! {
                    let mut #tmp = self.#field_name.to_database_values().into_iter();

                    result
                        #(#column_branches)*;
                }
            })
            .collect();

        vec![quote! {
            impl yukino::YukinoEntity for #name {
                fn table_name() -> &'static str {
                    #table_name
                }
            }

            impl yukino::view::EntityWithView for #name {
                type View = #view_name;
                type VerticalView = #vertical_view_name;

                fn all() -> yukino::query::QueryResultFilter<Self> {
                    yukino::query::QueryResultFilter::create()
                }
            }

            impl yukino::view::Value for #name {
                type L = #value_count;
                type ValueExprView = #view_name;

                fn converter() -> yukino::converter::ConverterRef<Self> where Self: Sized {
                    use yukino::converter::Converter;
                    #converter_name::instance()
                }
            }

            impl yukino::view::Insertable for #name {
                fn insert(self) -> yukino::query_builder::InsertQuery {
                    use yukino::view::Value;
                    let mut result = yukino::query_builder::Insert::into(#table_name.to_string());

                    #(#insert_branches)*

                    result
                }
            }
        }]
    }
}
