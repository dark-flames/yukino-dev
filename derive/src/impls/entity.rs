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
        let (count, iters, binds, columns): (usize, Vec<_>, Vec<_>, Vec<_>) =
            resolved.fields.iter().fold(
                (0, vec![], vec![], vec![]),
                |(mut c_count, mut c_iters, mut c_binds, mut c_columns), field| {
                    let field_name = &field.name;
                    let iter = format_ident!("{}_tmp", field.name.to_string().to_snake_case());
                    let (s, b, c): (usize, Vec<_>, Vec<_>) = field.definition.columns.iter().fold(
                        (0, vec![], vec![]),
                        |(mut c_s, mut c_b, mut c_c), c| {
                            let column_name = &c.name;
                            c_s += 1;

                            c_b.push(quote! {
                                let query = {
                                    use yukino::query_builder::BindArgs;
                                    yukino::query_builder::BindArgs::<'_, DB, O>::bind_args(
                                        #iter.next().unwrap(),
                                        query
                                    )
                                };
                            });

                            c_c.push(quote! {
                                #column_name.to_string()
                            });

                            (c_s, c_b, c_c)
                        },
                    );

                    c_iters.push(quote! {
                        let mut #iter = {
                            use yukino::view::Value;
                            self.#field_name.to_database_values().into_iter()
                        };
                    });
                    c_count += s;
                    c_binds.extend(b);
                    c_columns.extend(c);

                    (c_count, c_iters, c_binds, c_columns)
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

            impl<DB: sqlx::Database, O> yukino::view::Insertable<DB, O> for #name
                where yukino::query_builder::DatabaseValue: for<'p> yukino::query_builder::AppendToArgs<'p, DB> {
                type Entity = Self;
                type Source = Vec<Self>;

                fn insert(self) -> yukino::query_builder::InsertQuery<DB, O, Self::Source> {
                    use yukino::view::Value;
                    yukino::query_builder::Insert::into(
                        #table_name.to_string(),
                        <Self as yukino::view::Insertable<DB, O>>::columns(),
                        vec![self]
                    )
                }

                fn columns() -> Vec<String> where Self: Sized {
                    vec![#(#columns),*]
                }
            }

            impl<'q, DB: sqlx::Database, O> yukino::query_builder::ArgSource<'q, DB, O> for #name
                where yukino::query_builder::DatabaseValue: for<'p> yukino::query_builder::AppendToArgs<'p, DB> {
                fn insert_value_count() -> usize {
                    #count
                }

                fn bind_args(
                    self,
                    query: yukino::query_builder::QueryOf<'q, DB, O>
                ) ->  yukino::query_builder::QueryOf<'q, DB, O> where Self: Sized {
                    #(#iters)*

                    #(#binds)*

                    query
                }
            }
        }]
    }
}
