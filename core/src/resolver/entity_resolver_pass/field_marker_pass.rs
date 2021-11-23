use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use interface::DefinitionType;

use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};

pub struct FieldMakerPass();

impl EntityResolvePass for FieldMakerPass {
    fn instance() -> Box<dyn EntityResolvePass>
        where
            Self: Sized,
    {
        Box::new(FieldMakerPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![]
    }

    fn get_entity_implements(&mut self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let mod_name = &entity.marker_mod;
        let entity_name = format_ident!("{}", &entity.name);
        let markers: Vec<_> = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|field| {
                let marker_name = &field.marker_name;
                let field_name = &field.path.field_name;
                let field_type = &field.ty;
                quote! {
                    #[allow(non_camel_case_types)]
                    pub struct #marker_name();

                    impl FieldMarker for #marker_name {
                        type Entity = #entity_name;
                        type FieldType = #field_type;

                        fn field_name() -> &'static str {
                            #field_name
                        }

                        fn definition() -> &'static FieldDefinition {
                            Self::Entity::definition().fields.get(Self::field_name()).unwrap()
                        }
                    }
                }
            })
            .collect();

        vec![quote! {
            pub mod #mod_name {
                use yukino::{FieldMarker, YukinoEntity, FieldDefinition};
                use super::#entity_name;

                #(#markers)*
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
