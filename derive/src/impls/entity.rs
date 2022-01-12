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
        let (
            count,
            columns,
            deserialize_branches,
            serialize_tmp,
            serialize,
            where_branches,
            binding_body,
        ) =
            resolved.fields.iter().fold(
                (
                        0,
                        vec![],
                        vec![],
                        vec![],
                        quote! {
                            yukino::generic_array::arr![yukino::query_builder::DatabaseValue;]
                        },
                        vec![],
                        quote! {
                            query
                        }
                ),
                |(mut c_count, mut c_columns, mut de, mut ser_tmp, ser, mut w_branches, binding), field| {
                    let field_name = &field.name;
                    let field_ty = &field.ty;
                    let (s, c): (usize, Vec<_>) = field.definition.columns.iter().fold(
                        (0, vec![]),
                        |(mut c_s, mut c_c), c| {
                            let column_name = &c.name;
                            c_s += 1;

                            c_c.push(quote! {
                                #column_name.to_string()
                            });

                            (c_s, c_c)
                        },
                    );
                    c_columns.extend(c);
                    let offset = {
                        let type_num = format_ident!("U{}", c_count);
                        quote! {
                            yukino::generic_array::typenum::#type_num
                        }
                    };

                    de.push(quote! {
                        #field_name: yukino::view::DBMapping::<
                            'r,
                            DB,
                            yukino::generic_array::typenum::Sum<#offset, H>
                        >::from_result(values)?
                    });

                    ser_tmp.push(quote! {
                        let #field_name = yukino::view::Value::to_database_values(self.#field_name)
                    });

                    w_branches.push(
                        quote! {
                            #offset: std::ops::Add<H>,
                            yukino::generic_array::typenum::Sum<#offset, H>: yukino::view::ResultIndex,
                            #field_ty: yukino::view::DBMapping<'r, DB, yukino::generic_array::typenum::Sum<#offset, H>>
                        }
                    );


                    c_count += s;
                    (
                        c_count,
                        c_columns,
                        de,
                        ser_tmp,
                        quote! {
                            yukino::generic_array::sequence::Concat::concat(#ser, #field_name)
                        },
                        w_branches,
                        quote! {
                            yukino::view::DBMapping::<'r, DB, yukino::generic_array::typenum::Sum<#offset, H>>::bind_on_query(self.#field_name, #binding)
                        }
                    )
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

                fn to_database_values(self) -> yukino::generic_array::GenericArray<
                        yukino::query_builder::DatabaseValue,
                        Self::L
                > {
                    #(#serialize_tmp;)*
                    #serialize
                }
            }

            impl<'r, DB: sqlx::Database, H: yukino::view::ResultIndex> yukino::view::DBMapping<'r, DB, H> for #name
                where #(#where_branches),*
            {
                fn from_result(
                    values: &'r yukino::query_builder::RowOf<DB>
                ) -> yukino::view::ConvertResult<Self>
                    where Self: Sized
                {
                    Ok(#name {
                        #(#deserialize_branches),*
                    })
                }

                fn bind_on_query(
                    self,
                    query: yukino::query_builder::QueryOf<DB>
                ) -> yukino::query_builder::QueryOf<DB> where Self: Sized {
                    #binding_body
                }
            }

            impl<DB: sqlx::Database> yukino::view::Insertable<DB> for #name
                where Self: for<'r> yukino::view::DBMapping::<'r, DB, yukino::generic_array::typenum::U0> {
                type Entity = Self;
                type Source = Vec<Self>;

                fn insert(self) -> yukino::query_builder::InsertQuery<DB, Self::Source> {
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
                where Self: for<'r> yukino::view::DBMapping::<'r, DB, yukino::generic_array::typenum::U0> {
                fn insert_value_count() -> usize {
                    #count
                }

                fn bind_args(
                    self,
                    query: yukino::query_builder::QueryOf<'q, DB>
                ) ->  yukino::query_builder::QueryOf<'q, DB> where Self: Sized {
                    yukino::view::DBMapping::<DB, yukino::generic_array::typenum::U0>::bind_on_query(self, query)
                }
            }
        }]
    }
}
