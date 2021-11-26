use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};

pub struct EntityImplementPass {
    definitions: Vec<TokenStream>,
}

impl EntityResolvePass for EntityImplementPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(EntityImplementPass {
            definitions: vec![],
        })
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::{YukinoEntity, EntityDefinition};
            use yukino::view::{Value, View, EntityWithView, ValueCountOf};
            use yukino::query::{QueryResultFilter};
            use yukino::converter::ConverterRef;
            use yukino::lazy_static::lazy_static;
        }]
    }

    fn get_entity_implements(&mut self, entity: &ResolvedEntity) -> Vec<TokenStream> {
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
        let entity_id = &entity.id;
        self.definitions.push(quote! {
            (#entity_id, #name::definition())
        });

        vec![quote! {
            lazy_static! {
                static ref #definition_static_name: EntityDefinition = #definition;
            }
            impl YukinoEntity for #name {
                fn definition() -> &'static EntityDefinition {
                    &*#definition_static_name
                }

                fn entity_id() -> usize {
                    #entity_id
                }
            }

            impl EntityWithView for #name {
                type View = #view_name;

                fn all() -> QueryResultFilter<Self> {
                    QueryResultFilter::create(&*DEFINITION_MANAGER)
                }
            }

            impl Value for #name {
                type L = typenum::#value_count;
                type ValueExprView = #view_name;

                fn converter() -> ConverterRef<Self> where Self: Sized {
                    #converter_name::instance()
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        let items = &self.definitions;
        vec![quote! {
            use yukino::DefinitionManager;
            lazy_static! {
                static ref DEFINITION_MANAGER: DefinitionManager = DefinitionManager::create(
                    vec![
                        #(#items),*
                    ]
                )
            }
        }]
    }
}
