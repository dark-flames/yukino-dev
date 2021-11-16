use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct EntityImplementPass();

impl EntityResolvePass for EntityImplementPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(EntityImplementPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::{YukinoEntity, EntityDefinition};
            use yukino::view::{Value, View, EntityWithView };
            use yukino::converter::ConverterRef;
            use yukino::generic_array::functional::FunctionalSequence;
            use yukino::lazy_static::lazy_static;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let name = format_ident!("{}", &entity.name);
        let view_name = &entity.view_name;
        let converter_name = &entity.converter_name;
        let value_count = format_ident!(
            "U{}",
            entity
                .fields
                .iter()
                .fold(0, |c, i| c + i.converter_param_count)
        );
        let definition_static_name =
            format_ident!("{}_DEFINITION", entity.name.to_snake_case().to_uppercase());
        let definition = &entity.entity_definition;

        vec![quote! {
            lazy_static! {
                static ref #definition_static_name: EntityDefinition = #definition;
            }
            impl YukinoEntity for #name {
                fn definition() -> &'static EntityDefinition {
                    &*#definition_static_name
                }
            }

            impl EntityWithView for #name {
                type View = #view_name;
            }

            impl Value for #name {
                type L = typenum::#value_count;

                fn converter() -> ConverterRef<Self> where Self: Sized {
                    #converter_name::instance()
                }

                fn view(&self) -> ExprViewBox<Self>
                    where
                        Self: Sized {
                    Box::new(#view_name::from_exprs(
                        Self::converter().serialize(self).unwrap().map(Expr::Lit),
                    ))
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
