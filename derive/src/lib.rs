use proc_macro::TokenStream;

use syn::parse_macro_input;

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
