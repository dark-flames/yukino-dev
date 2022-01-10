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
        let (fields, iters, values, columns): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = resolved.fields.iter().fold(
            (vec![], vec![], vec![], vec![]),
            |(mut c_fields, mut c_iters, mut c_values, mut c_columns), field| {
                let primary_field = field.primary;

                if !primary_field {
                    let field_name = &field.name;
                    let iter = format_ident!("{}_tmp", field.name.to_string().to_snake_case());
                    let ty = &field.ty;
                    let (v, c): (Vec<_>, Vec<_>) = field
                        .definition
                        .columns
                        .iter()
                        .filter_map(|c| {
                            let column_name = &c.name;
                            if primary_field {
                                None
                            } else {
                                Some((
                                    quote! {
                                        yukino::query_builder::AssignmentValue::Expr(
                                            yukino::query_builder::Expr::Lit(#iter.next().unwrap())
                                        )
                                    },quote! {
                                        #column_name.to_string()
                                    }
                                ))
                            }

                        })
                        .unzip();
                    c_fields.push(quote! {pub #field_name: #ty});

                    c_iters.push(quote! {
                        let mut #iter = {
                            use yukino::view::Value;
                            self.#field_name.to_database_values().into_iter()
                        };
                    });

                    c_values.extend(v);
                    c_columns.extend(c);
                }

                (c_fields, c_iters, c_values, c_columns)
            },
        );

        vec![quote! {
            pub struct #name {
                #(#fields),*
            }

            impl yukino::view::Insertable for #name {
                type Entity = #entity_name;
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
