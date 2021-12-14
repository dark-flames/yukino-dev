use proc_macro2::TokenStream;
use quote::quote;

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct FieldMarkerImplementor;

impl Implementor for FieldMarkerImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let marker_mod = &resolved.marker_mod;
        let entity_name = &resolved.entity_name;
        let markers: Vec<_> = resolved.fields.iter().map(
            |f| {
                let marker_name = &f.field_marker;
                let ty = &f.ty;
                let view_tags = &f.tag_list;
                let fields_name = &f.name;
                let columns: Vec<_> = f.definition.columns.iter().map(
                    |d| &d.name
                ).collect();
                quote! {
                    pub struct #marker_name;

                    impl yukino::view::FieldMarker for #marker_name {
                        type Entity = super::#entity_name;
                        type FieldType = #ty;
                        type ViewTags = #view_tags;

                        fn columns() -> yukino::generic_array::GenericArray<
                            String,
                            <Self::FieldType as yukino::view::Value>::L
                        > where Self: Sized {
                            yukino::generic_array::arr![String; #(#columns.to_string()),*]
                        }

                        fn view(entity_view: <Self::Entity as yukino::view::EntityWithView>::View)
                            -> yukino::view::ExprViewBoxWithTag<Self::FieldType, Self::ViewTags> {
                            entity_view.#fields_name
                        }
                    }
                }
            }
        ).collect();

        vec![quote! {
            pub mod #marker_mod {
                #(#markers)*
            }
        }]
    }
}