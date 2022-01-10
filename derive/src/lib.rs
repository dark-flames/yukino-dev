use proc_macro::TokenStream;

use proc_macro2::TokenStream  as TokenStream2;
use quote::quote;
use syn::{Error, ExprTuple, parse_macro_input, parse_quote, TypeTuple};

use crate::entity::EntityResolver;
use crate::fields::{BasicFieldResolver, DateTimeFieldResolver, DecimalFieldResolver};
use crate::impls::{
    AssociationImplementor, ConverterImplementor, EntityImplementor, FieldMarkerImplementor,
    PrimaryImplementor, ViewImplementor,
};

mod entity;
mod fields;
mod impls;
mod resolved;

#[proc_macro_derive(Entity, attributes(name, belongs_to, auto_increment, id))]
pub fn derive_entity(tokens: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(tokens as syn::ItemStruct);
    let resolver = EntityResolver::create(
        vec![
            Box::new(BasicFieldResolver),
            Box::new(DecimalFieldResolver),
            Box::new(DateTimeFieldResolver),
        ],
        vec![
            Box::new(ConverterImplementor),
            Box::new(EntityImplementor),
            Box::new(ViewImplementor),
            Box::new(PrimaryImplementor),
            Box::new(AssociationImplementor),
            Box::new(FieldMarkerImplementor),
        ],
    );

    let result = resolver
        .get_implements(&item_struct)
        .unwrap_or_else(|err| err.to_compile_error());

    //println!("{}", result);

    result.into()
}

#[proc_macro]
pub fn tuple(tokens: TokenStream) -> TokenStream {
    let tokens2: TokenStream2 = tokens.into();
    let tuple: TypeTuple = parse_quote! {
        (#tokens2)
    };

    if tuple.elems.len() < 2 {
        Error::new_spanned(tuple, "tuple! need at least 2 parameters").to_compile_error()
    } else {
        let mut iter = tuple.elems.into_iter();
        let (first, second) = (iter.next().unwrap(), iter.next().unwrap());

        iter.fold(
            quote! {
                (#first, #second)
            },
            |c, i| {
                quote! {
                    (#c, #i)
                }
            }
        )
    }.into()
}

#[proc_macro]
pub fn make_tuple(tokens: TokenStream) -> TokenStream {
    let tokens2: TokenStream2 = tokens.into();
    let tuple: ExprTuple = parse_quote! {
        (#tokens2)
    };

    if tuple.elems.len() < 2 {
        Error::new_spanned(tuple, "make_tuple! need at least 2 parameters").to_compile_error()
    } else {
        let mut iter = tuple.elems.into_iter();
        let (first, second) = (iter.next().unwrap(), iter.next().unwrap());

        iter.fold(
            quote! {
                Box::new(TupleExprView::from((#first, #second))) as yukino::view::ExprViewBoxWithTag<_, _>
            },
            |c, i| {
                quote! {
                    Box::new(TupleExprView::from((#c, #i))) as yukino::view::ExprViewBoxWithTag<_, _>
                }
            }
        )
    }.into()
}
