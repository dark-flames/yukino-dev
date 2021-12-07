use proc_macro2::{Ident, TokenStream};
use syn::Path;

use interface::FieldDefinition;

pub struct ResolvedEntity {
    pub table_name: String,
    pub entity_name: Ident,
    pub view_name: Ident,
    pub vertical_name: Ident,
    pub converter_name: Ident,
    pub fields: Vec<ResolvedField>,
    pub associations: Vec<ResolvedAssociation>,
}

pub struct ResolvedAssociation {
    pub ref_entity_path: Path,
    pub foreign_key: Ident,
    pub column_name: String,
    pub ty: TokenStream,
}

pub struct ResolvedField {
    pub name: Ident,
    pub definition: FieldDefinition,
    pub ty: TokenStream,
    pub view_construct: TokenStream,
    pub view_ty: TokenStream,
    pub view_full_path: TokenStream,
    pub vertical_ty: TokenStream,
    pub vertical_full_path: TokenStream,
    pub converter_ty: TokenStream,
    pub converter_value_count: usize,
}