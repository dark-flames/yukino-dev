use proc_macro2::TokenStream;
use quote::quote;

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct PrimaryImplementor;

impl Implementor for PrimaryImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        resolved.fields.iter().find(|f| f.definition.primary_key).map(
            |field| {
                let entity_name = &resolved.entity_name;
                let view_name = &resolved.view_name;
                let column_name = &field.definition.identity_column;
                let field_name = &field.name;
                let field_type = &field.ty;
                let tags = &field.tag_list;
                quote! {
                    impl yukino::WithPrimaryKey for #entity_name {
                        type Type = #field_type;
                        fn primary_key(&self) -> &Self::Type {
                            &self.#field_name
                        }

                        fn primary_key_name() -> &'static str where Self: Sized {
                            #column_name
                        }
                    }

                    impl yukino::view::ViewWithPrimaryKey for #view_name {
                        type Type = #field_type;
                        type PrimaryKeyTag = #tags;

                        fn primary_key(&self) -> &yukino::view::ExprViewBoxWithTag<Self::Type, Self::PrimaryKeyTag> {
                            &self.#field_name
                        }
                    }
                }
            }
        ).into_iter().collect()
    }
}