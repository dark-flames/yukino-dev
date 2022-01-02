use proc_macro2::TokenStream;
use quote::quote;

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct AssociationImplementor;

impl Implementor for AssociationImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let entity_name = &resolved.entity_name;
        let view_name = &resolved.view_name;
        resolved
            .associations
            .iter()
            .map(|assoc| {
                let target_entity_name = &assoc.ref_entity_path;
                let ty = &assoc.ty;
                let field_name = &assoc.foreign_key;
                let column_name = &assoc.column_name;
                let tags = &resolved.fields.iter().find(|f| f.name.eq(field_name)).unwrap().tag_list;
                quote! {
                    impl yukino::Association<#target_entity_name> for #entity_name {
                        type ForeignKeyType = #ty;
                        fn foreign_key(&self) -> &Self::ForeignKeyType {
                            &self.#field_name
                        }

                        fn foreign_key_name() -> &'static str where Self: Sized {
                            #column_name
                        }
                    }

                    impl yukino::view::AssociatedView<#target_entity_name> for #view_name {
                        type ForeignKeyType = #ty;
                        type ForeignKeyTags = #tags;
                        fn foreign_key(&self) -> &yukino::view::ExprBoxOfAssociatedView<Self, #target_entity_name> {
                            &self.#field_name
                        }
                    }
                }
            })
            .collect()
    }
}
