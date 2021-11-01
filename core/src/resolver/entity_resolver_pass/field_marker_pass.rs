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
        let mod_name = format_ident!("{}", entity.name.to_snake_case());
        let view_name = format_ident!("{}View", &entity.name);
        let markers: Vec<_> = entity
            .fields
            .values()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .map(|field| {
                let marker_name = &field.marker_name;
                let field_ident = format_ident!("{}", field.path.field_name);
                let converter = &field.converter;
                let field_name = &field.path.field_name;
                let ty = &field.value_type;
                let definition = &field.definition;
                let converter_static_name = format_ident!(
                    "{}_CONVERTER",
                    field.path.field_name.to_snake_case().to_uppercase()
                );
                let definition_static_name = format_ident!(
                    "{}_DEFINITION",
                    field.path.field_name.to_snake_case().to_uppercase()
                );
                let converter_type = &field.converter_type;
                quote! {
                    #[allow(non_camel_case_types)]
                    pub struct #marker_name();
                    lazy_static! {
                        static ref #converter_static_name: #converter_type = #converter;
                        static ref #definition_static_name: FieldDefinition = #definition;
                    }

                    impl FieldMarker for #marker_name {
                        type ValueType = #ty;

                        fn field_name() -> &'static str {
                            #field_name
                        }

                        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
                            &*#converter_static_name
                        }

                        fn definition() -> &'static FieldDefinition {
                            &*#definition_static_name
                        }

                        fn view() -> &'static Expr<Self::ValueType> {
                            &#view_name::static_ref().#field_ident
                        }
                    }
                }
            })
            .collect();

        vec![quote! {
            pub mod #mod_name {
                use yukino::interface::FieldMarker;
                use yukino::interface::def::FieldDefinition;
                use yukino::converter::Converter;
                use yukino::expr::Expr;
                use yukino::interface::EntityView;
                use lazy_static::lazy_static;
                use super::#view_name;

                #(#markers)*
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
