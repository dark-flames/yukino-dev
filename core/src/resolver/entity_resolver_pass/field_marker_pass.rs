use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let mod_name = &entity.marker_mod;
        let markers: Vec<_> = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|field| {
                let marker_name = &field.marker_name;
                let field_name = &field.path.field_name;
                let ty = &field.ty;
                let definition = &field.definition;
                let definition_static_name = format_ident!(
                    "{}_DEFINITION",
                    field.path.field_name.to_snake_case().to_uppercase()
                );
                quote! {
                    #[allow(non_camel_case_types)]
                    pub struct #marker_name();
                    lazy_static! {
                        static ref #definition_static_name: FieldDefinition = #definition;
                    }

                    impl FieldMarker for #marker_name {
                        type ValueType = #ty;

                        fn field_name() -> &'static str {
                            #field_name
                        }

                        fn definition() -> &'static FieldDefinition {
                            &*#definition_static_name
                        }
                    }
                }
            })
            .collect();

        vec![quote! {
            pub mod #mod_name {
                use yukino::interface::FieldMarker;
                use yukino::interface::def::FieldDefinition;
                use lazy_static::lazy_static;

                #(#markers)*
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
