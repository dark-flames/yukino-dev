use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
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
            use yukino::interface::Entity;
            use yukino::view::{Value, View};
            use yukino::converter::ConverterRef;
            use yukino::generic_array::functional::FunctionalSequence;
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
        vec![quote! {
            impl Entity for #name {
                type View = #view_name;
            }

            impl Value for #name {
                type L = typenum::#value_count;

                fn converter() -> ConverterRef<Self, Self::L> where Self: Sized {
                    #converter_name::instance()
                }

                fn view(&self) -> ExprViewBox<Self, Self::L>
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
