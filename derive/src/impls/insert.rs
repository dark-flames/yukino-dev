use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct InsertImplementor;

impl Implementor for InsertImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let entity_name = &resolved.entity_name;
        let name = &resolved.new_entity_name;
        let table_name = &resolved.table_name;
        let (count, fields, iters, binds, columns): (usize, Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
            resolved.fields.iter().fold(
                (0, vec![], vec![], vec![], vec![]),
                |(mut c_count, mut c_fields, mut c_iters, mut c_values, mut c_columns), field| {
                    let primary_field = field.primary;

                    if !primary_field {
                        let field_name = &field.name;
                        let iter = format_ident!("{}_tmp", field.name.to_string().to_snake_case());
                        let ty = &field.ty;
                        let (s, b, c): (usize, Vec<_>, Vec<_>) = field
                            .definition
                            .columns
                            .iter()
                            .fold((0, vec![], vec![]), |(mut c_s, mut c_b, mut c_c), c| {
                                let column_name = &c.name;
                                c_s += 1;

                                c_b.push(quote! {
                                    let query = {
                                        use yukino::query_builder::BindArgs;
                                        yukino::query_builder::BindArgs::<'_, DB>::bind_args(
                                            #iter.next().unwrap(),
                                            query
                                        )
                                    };
                                });

                                c_c.push(quote! {
                                    #column_name.to_string()
                                });

                                (c_s, c_b, c_c)
                            });
                        c_fields.push(quote! {pub #field_name: #ty});

                        c_iters.push(quote! {
                            let mut #iter = {
                                use yukino::view::Value;
                                self.#field_name.to_database_values().into_iter()
                            };
                        });

                        c_values.extend(b);
                        c_columns.extend(c);
                        c_count += s;
                    }

                    (c_count, c_fields, c_iters, c_values, c_columns)
                },
            );

        vec![quote! {
            pub struct #name {
                #(#fields),*
            }

            impl<DB: sqlx::Database> yukino::view::Insertable<DB> for #name
                where yukino::query_builder::DatabaseValue: for<'p> yukino::query_builder::AppendToArgs<'p, DB> {
                type Entity = #entity_name;
                type Source = Vec<Self>;

                fn insert(self) -> yukino::query_builder::InsertQuery<DB, Self::Source>  where Self: Sized {
                    use yukino::view::Value;
                    yukino::query_builder::Insert::into(
                        #table_name.to_string(),
                        <Self as yukino::view::Insertable<DB>>::columns(),
                        vec![self]
                    )
                }

                fn columns() -> Vec<String> where Self: Sized {
                    vec![#(#columns),*]
                }
            }

            impl<'q, DB: sqlx::Database> yukino::query_builder::ArgSource<'q, DB> for #name
                where yukino::query_builder::DatabaseValue: for<'p> yukino::query_builder::AppendToArgs<'p, DB> {
                fn insert_value_count() -> usize {
                    #count
                }

                fn bind_args(
                    self,
                    query: yukino::query_builder::QueryOf<'q, DB>
                ) ->  yukino::query_builder::QueryOf<'q, DB> where Self: Sized {
                    #(#iters)*

                    #(#binds)*

                    query
                }
            }
        }]
    }
}
