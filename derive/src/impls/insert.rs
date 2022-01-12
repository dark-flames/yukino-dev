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
        let (count, fields, columns, where_clauses, binds) = resolved.fields.iter().fold(
                (0, vec![], vec![], vec![], quote! {query}),
                |(mut c_count, mut c_fields, mut c_columns, mut c_wheres, mut c_binds), field| {
                    let primary_field = field.primary;

                    if !primary_field {
                        let field_name = &field.name;
                        let ty = &field.ty;
                        let (s, c): (usize, Vec<_>) = field
                            .definition
                            .columns
                            .iter()
                            .fold((0, vec![]), |(mut c_s, mut c_c), c| {
                                let column_name = &c.name;
                                c_s += 1;

                                c_c.push(quote! {
                                    #column_name.to_string()
                                });

                                (c_s, c_c)
                            });

                        let offset = {
                            let type_num = format_ident!("U{}", c_count);
                                quote! {
                                yukino::generic_array::typenum::#type_num
                            }
                        };

                        c_fields.push(quote! {pub #field_name: #ty});

                        c_columns.extend(c);
                        c_count += s;
                        c_wheres.push(quote! {
                            #ty: for<'r> yukino::view::DBMapping<'r, DB, #offset>
                        });

                        c_binds = quote! {
                            yukino::view::DBMapping::<DB, #offset>::bind_on_query(self.#field_name, #c_binds)
                        };
                    }

                    (c_count, c_fields, c_columns, c_wheres, c_binds)
                },
            );

        vec![quote! {
            #[derive(Clone, Debug)]
            pub struct #name {
                #(#fields),*
            }

            impl<DB: sqlx::Database> yukino::view::Insertable<DB> for #name
                where Self: for<'q> yukino::query_builder::ArgSource<'q, DB>  {
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
                where #(#where_clauses),* {
                fn insert_value_count() -> usize {
                    #count
                }

                fn bind_args(
                    self,
                    query: yukino::query_builder::QueryOf<'q, DB>
                ) ->  yukino::query_builder::QueryOf<'q, DB> where Self: Sized {
                    #binds
                }
            }
        }]
    }
}
