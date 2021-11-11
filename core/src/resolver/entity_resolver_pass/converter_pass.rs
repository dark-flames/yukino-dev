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
            use yukino::converter::{Converter, ConvertResult, Deserializer};
            use yukino::generic_array::typenum;
            use yukino::generic_array::sequence::{Concat, Split};
            use yukino::generic_array::{GenericArray, arr};
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
        let iter = entity
            .fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .enumerate();
        let field_count = iter.clone().last().unwrap().0;
        let (deserialize_tmp, deserialize_branches, serialize_tmp, serialize) = iter
            .fold(
                (
                    vec![],
                    vec![],
                    vec![],
                    quote! {
                        arr![DatabaseValue;]
                    },
                ),
                |(mut de_tmp, mut de, mut ser_tmp, ser), (index, f)| {
                    let field_name = format_ident!("{}", f.path.field_name);
                    let field_ty = &f.ty;
                    let field_value_count = format_ident!("U{}", f.converter_param_count);

                    if index == field_count {
                        de_tmp.push(quote! {
                            let (#field_name, _) = Split::<_, typenum::#field_value_count>::split(rest)
                        });
                    } else {
                        de_tmp.push(quote! {
                            let (#field_name, rest) = Split::<_, typenum::#field_value_count>::split(rest)
                        });
                    }

                    de.push(quote! {
                        #field_name: (*<#field_ty>::converter().deserializer())(#field_name)?
                    });

                    ser_tmp.push(quote! {
                        let #field_name = <#field_ty>::converter().serialize(&value.#field_name)?
                    });

                    (
                        de_tmp,
                        de,
                        ser_tmp,
                        quote! {
                            Concat::concat(#ser, #field_name)
                        },
                    )
                },
            );
        let value_count = format_ident!("U{}", entity.value_count);
        vec![quote! {
            #[derive(Clone)]
            pub struct #converter_name;

            static #static_name: #name = #converter_name;

            impl Converter<typenum::#value_count> for #name {
                type Output = #entity_name;

                fn instance() -> &'static Self where Self: Sized {
                    &#static_name
                }

                fn deserializer(&self) -> Deserializer<Self::Output, typenum::#value_count> {
                    Box::new(|rest| {
                        #(#deserialize_tmp;)*
                        Ok(#entity_name {
                            #(#deserialize_branches),*
                        })
                    })
                }

                fn serialize(&self, value: &Self::Output) -> ConvertResult<GenericArray<DatabaseValue, typenum::#value_count>> {
                    #(#serialize_tmp;)*
                    Ok(#serialize)
                }
            }
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
