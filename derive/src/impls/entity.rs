use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::impls::Implementor;
use crate::resolved::ResolvedEntity;

pub struct EntityImplementor;

impl Implementor for EntityImplementor {
    fn get_implements(&self, resolved: &ResolvedEntity) -> Vec<TokenStream> {
        let name = &resolved.entity_name;
        let view_name = &resolved.view_name;
        let vertical_view_name = &resolved.vertical_name;
        let converter_name = &resolved.converter_name;
        let table_name = &resolved.table_name;
        let value_count = {
            let type_num = format_ident!(
                "U{}",
                resolved.fields
                    .iter()
                    .fold(0, |c, i| c + i.converter_value_count)
            );

            quote! {
                yukino::generic_array::typenum::#type_num
            }
        };

        vec![quote! {
            impl yukino::YukinoEntity for #name {
                fn table_name() -> &'static str {
                    #table_name
                }
            }

            impl yukino::view::EntityWithView for #name {
                type View = #view_name;
                type VerticalView = #vertical_view_name;

                fn all() -> yukino::query::QueryResultFilter<Self> {
                    yukino::query::QueryResultFilter::create()
                }
            }

            impl yukino::view::Value for #name {
                type L = #value_count;
                type ValueExprView = #view_name;

                fn converter() -> yukino::converter::ConverterRef<Self> where Self: Sized {
                    use yukino::converter::Converter;
                    #converter_name::instance()
                }
            }
        }]
    }
}