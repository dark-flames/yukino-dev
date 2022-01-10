use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct EntityImplementor;

impl Implementor for EntityImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let name = &resolved.entity_name;
        let new_entity_name = &resolved.new_entity_name;
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
        let (iters, values, columns): (Vec<_>, Vec<_>, Vec<_>) = resolved.fields.iter().fold(
            (vec![], vec![], vec![]),
            |(mut c_iters, mut c_values, mut c_columns), field| {
                let field_name = &field.name;
                let iter = format_ident!("{}_tmp", field.name.to_string().to_snake_case());
                let (v, c): (Vec<_>, Vec<_>) = field
                    .definition
                    .columns
                    .iter()
                    .map(|c| {
                        let column_name = &c.name;
                        (
                            quote! {
                                yukino::query_builder::AssignmentValue::Expr(
                                    yukino::query_builder::Expr::Lit(#iter.next().unwrap())
                                )
                            },
                            quote! {
                                #column_name.to_string()
                            },
                        )
                    })
                    .unzip();

                c_iters.push(quote! {
                    let mut #iter = {
                        use yukino::view::Value;
                        self.#field_name.to_database_values().into_iter()
                    };
                });
                c_values.extend(v);
                c_columns.extend(c);

                (c_iters, c_values, c_columns)
            },
        );

        vec![quote! {
            impl yukino::YukinoEntity for #name {
                fn table_name() -> &'static str {
                    #table_name
                }
            }

            impl yukino::view::EntityWithView for #name {
                type View = #view_name;
                type VerticalView = #vertical_view_name;
                type New = #new_entity_name;

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
                type Entity = Self;
                fn insert(self) -> yukino::query_builder::InsertQuery {
                    use yukino::view::Value;
                    let mut result = yukino::query_builder::Insert::into(
                        #table_name.to_string(),
                        Self::columns()
                    );

                    result.append(self.values());

                    result
                }

                fn columns() -> Vec<String> where Self: Sized {
                    vec![#(#columns),*]
                }

                fn values(&self) -> Vec<yukino::query_builder::AssignmentValue> {
                    #(#iters)*

                    vec![#(#values),*]
                }
            }
        }]
    }
}
