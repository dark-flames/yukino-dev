use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct ConverterImplementor;

impl Implementor for ConverterImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let entity_name = &resolved.entity_name;
        let name = &resolved.converter_name;
        let last_index = resolved.fields.len() - 1;
        let (deserialize_tmp, deserialize_branches, serialize_tmp, serialize) =
            resolved.fields.iter().enumerate().fold(
                (
                    vec![],
                    vec![],
                    vec![],
                    quote! {
                        yukino::generic_array::arr![yukino::query_builder::DatabaseValue;]
                    },
                ),
                |(mut de_tmp, mut de, mut ser_tmp, ser), (index, f)| {
                    let field_name = &f.name;
                    let field_ty = &f.ty;
                    let field_value_count = {
                        let type_num = format_ident!("U{}", f.converter_value_count);
                        quote! {
                            yukino::generic_array::typenum::#type_num
                        }
                    };

                    if index == last_index {
                        de_tmp.push(quote! {
                            let (#field_name, _) = yukino::generic_array::sequence::Split::<
                                _, #field_value_count
                            >::split(rest)
                        });
                    } else {
                        de_tmp.push(quote! {
                            let (#field_name, rest) = yukino::generic_array::sequence::Split::<
                                _, #field_value_count
                            >::split(rest)
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
                            yukino::generic_array::sequence::Concat::concat(#ser, #field_name)
                        },
                    )
                },
            );

        vec![quote! {
            #[derive(Clone)]
            pub struct #name;

            impl yukino::converter::Converter for #name {
                type Output = #entity_name;

                fn instance() -> &'static Self where Self: Sized {
                    use yukino::converter::ConverterInstance;
                    &Self::INSTANCE
                }

                fn deserializer(&self) -> yukino::converter::Deserializer<Self::Output> {
                    use yukino::converter::Converter;
                    use yukino::view::Value;
                    Box::new(|rest| {
                        #(#deserialize_tmp;)*
                        Ok(#entity_name {
                            #(#deserialize_branches),*
                        })
                    })
                }

                fn serialize(&self, value: &Self::Output)
                    -> yukino::converter::ConvertResult<
                        yukino::generic_array::GenericArray<
                            yukino::query_builder::DatabaseValue,
                            yukino::view::ValueCountOf<Self::Output>>
                > {
                    use yukino::view::Value;
                    #(#serialize_tmp;)*
                    Ok(#serialize)
                }
            }

            impl yukino::converter::ConverterInstance for #name {
                const INSTANCE: Self = #name;
            }
        }]
    }
}
