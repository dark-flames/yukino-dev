use crate::interface::def::DefinitionType;
use crate::resolver::entity::{EntityResolvePass, ResolvedEntity};
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
            use yukino::db::ty::DatabaseValue;
        }]
    }

    fn get_entity_implements(&self, entity: &ResolvedEntity) -> Vec<TokenStream> {
        let entity_name = format_ident!("{}", &entity.name);
        let name = &entity.converter_name;
        let converter_name = &entity.converter_name;
        let marker_mod = &entity.marker_mod;
        let (_, deserialize_branches, serialize_branches): (usize, Vec<_>, Vec<_>) = entity.fields
            .iter()
            .filter(|f| f.definition.definition_ty != DefinitionType::Generated)
            .fold(
                (0, vec![], vec![]),
                |(count, mut de, mut ser), f| {
                    let marker_name = &f.marker_name;
                    let field_name = format_ident!("{}", f.path.field_name);
                    let ending = count + f.converter_param_count;

                    de.push(quote! {
                        #field_name: (*#marker_mod::#marker_name::converter().deserializer())(&v[#count..#ending])?
                    });

                    ser.push(quote! {
                        #marker_mod::#marker_name::converter().serialize(&value.#field_name)?
                    });

                    (ending, de, ser)
                }
            );

        vec![quote! {
            #[derive(Clone)]
            pub struct #converter_name();

            unsafe impl Sync for #converter_name {}

            impl Converter for #name {
                type Output = #entity_name;

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

            impl Value for #name {}
        }]
    }

    fn get_additional_implements(&self) -> Vec<TokenStream> {
        vec![]
    }
}
