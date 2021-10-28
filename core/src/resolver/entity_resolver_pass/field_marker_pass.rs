use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Type};

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
        let mod_name = format_ident!("{}", entity.name.to_snake_case());

        let markers: Vec<_> = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|field| {
                let marker_name = format_ident!("{}", field.path.field_name.to_snake_case());
                let converter = &field.converter;
                let field_name = &field.path.field_name;
                let ty: Type = parse_str(field.definition.ty.as_str()).unwrap();
                let definition = &field.definition;
                let converter_static_name =
                    format_ident!("{}_CONVERTER", field.path.field_name.to_uppercase());
                let definition_static_name =
                    format_ident!("{}_DEFINITION", field.path.field_name.to_uppercase());
                let converter_type = &field.converter_type;
                quote! {
                    #[allow(non_camel_case_types)]
                    pub struct #marker_name();
                    lazy_static! {
                        static ref #converter_static_name: #converter_type = #converter;
                        static ref #definition_static_name: FieldDefinition = #definition;
                    }


                    impl FieldMarker for #marker_name {
                        type Type = #ty;

                        fn field_name() -> &'static str {
                            #field_name
                        }

                        fn data_converter() -> &'static dyn DataConverter<FieldType = Self::Type> {
                            &*#converter_static_name
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
                use yukino::interface::converter::DataConverter;
                use yukino::resolver::field_resolve_cells::numeric::*;
                use lazy_static::lazy_static;

                #(#markers)*
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
