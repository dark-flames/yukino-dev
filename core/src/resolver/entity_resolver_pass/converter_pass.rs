use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct ConverterPass();

impl EntityResolvePass for ConverterPass {
    fn instance() -> Box<dyn EntityResolvePass>
    where
        Self: Sized,
    {
        Box::new(ConverterPass())
    }

    fn get_dependencies(&self) -> Vec<TokenStream> {
        vec![quote! {
            use yukino::converter::Converter;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let entity_name = format_ident!("{}", &entity.name);
        let name = &entity.converter_name;
        let converter_name = &entity.converter_name;
        let static_name = format_ident!(
            "{}",
            converter_name.to_string().to_snake_case().to_uppercase()
        );
        let (param_count, deserialize_branches, serialize_branches): (usize, Vec<_>, Vec<_>) =
            entity
                .fields
                .iter()
                .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
                .fold((0, vec![], vec![]), |(count, mut de, mut ser), f| {
                    let field_name = format_ident!("{}", f.path.field_name);
                    let ending = count + f.converter_param_count;
                    let field_ty = &f.ty;

                    de.push(quote! {
                        #field_name: (*<#field_ty>::converter().deserializer())(&v[#count..#ending])?
                    });

                    ser.push(quote! {
                        <#field_ty>::converter().serialize(&value.#field_name)?
                    });

                    (ending, de, ser)
                });

        vec![quote! {
            #[derive(Clone)]
            pub struct #converter_name;

            unsafe impl Sync for #converter_name {}

            static #static_name: #name = #converter_name;

            impl Converter for #name {
                type Output = #entity_name;

                fn instance() -> &'static Self where Self: Sized {
                    &#static_name
                }

                fn param_count(&self) -> usize {
                    #param_count
                }

                fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>> {
                    Box::new(|v| {
                        Ok(#entity_name {
                            #(#deserialize_branches),*
                        })
                    })
                }

                fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>> {
                    Ok(vec![
                        #(#serialize_branches),*
                    ].into_iter().flatten().collect())
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
