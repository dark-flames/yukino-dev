use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, ItemStruct, LitStr, Meta, NestedMeta};
use syn::Result;

use crate::fields::FieldResolver;
use crate::impls::Implementor;
use crate::resolved::{ResolvedAssociation, ResolvedEntity};

pub struct EntityResolver {
    field_resolvers: Vec<Box<dyn FieldResolver>>,
    implementors: Vec<Box<dyn Implementor>>,
}

impl EntityResolver {
    pub fn create(
        field_resolvers: Vec<Box<dyn FieldResolver>>,
        implementors: Vec<Box<dyn Implementor>>,
    ) -> Self {
        EntityResolver {
            field_resolvers,
            implementors,
        }
    }

    pub fn resolve(&self, ast: &ItemStruct) -> Result<ResolvedEntity> {
        let entity_name = ast.ident.clone();
        let table_name = ast
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("name"))
            .map(|attr| attr.parse_args::<LitStr>().map(|lit| lit.value()))
            .unwrap_or_else(|| Ok(ast.ident.to_string().to_snake_case()))?;

        let fields = if let Fields::Named(name_fields) = &ast.fields {
            name_fields
                .named
                .iter()
                .map(|field| {
                    self.field_resolvers
                        .iter()
                        .find(|resolver| resolver.can_resolve(field))
                        .ok_or_else(|| {
                            Error::new_spanned(field, "Cannot find a field resolver for this field")
                        })
                        .and_then(|resolver| resolver.resolve_field(field))
                })
                .collect::<Result<Vec<_>>>()
        } else {
            return Err(Error::new_spanned(ast, "Expected named fields"));
        }?;

        let associations = ast
            .fields
            .iter()
            .filter_map(|f| {
                f.attrs
                    .iter()
                    .find(|attr| attr.path.is_ident("belongs_to"))
                    .map(|attr| match attr.parse_meta()? {
                        Meta::List(l) => {
                            let ref_entity_path = l
                                .nested
                                .iter()
                                .next()
                                .ok_or_else(|| {
                                    Error::new_spanned(attr, "Expected referenced entity path")
                                })
                                .and_then(|meta| match meta {
                                    NestedMeta::Meta(Meta::Path(p)) => Ok(p.clone()),
                                    _ => Err(Error::new_spanned(
                                        attr,
                                        "Expected referenced entity path",
                                    )),
                                })?;

                            let foreign_key = f.ident.clone().unwrap();

                            fields
                                .iter()
                                .find(|f| f.name == foreign_key)
                                .ok_or_else(|| {
                                    Error::new_spanned(attr, "Cannot find a field with this name")
                                })
                                .map(|field| ResolvedAssociation {
                                    ref_entity_path,
                                    foreign_key,
                                    column_name: field.definition.identity_column.clone(),
                                    ty: field.ty.clone(),
                                })
                        }
                        _ => Err(Error::new_spanned(
                            attr,
                            "`belongs_to` attribute must be a list",
                        )),
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        if fields.iter().filter(|f| f.definition.primary_key).count() > 1 {
            return Err(Error::new_spanned(
                ast,
                "Only one field can be marked as primary key",
            ));
        }

        Ok(ResolvedEntity {
            table_name,
            view_name: format_ident!("{}View", entity_name),
            vertical_name: format_ident!("Vertical{}View", entity_name),
            converter_name: format_ident!("{}Converter", entity_name),
            marker_mod: format_ident!("{}", entity_name.to_string().to_snake_case()),
            entity_name,
            fields,
            associations,
        })
    }

    pub fn get_implements(&self, ast: &ItemStruct) -> Result<TokenStream> {
        let implements: Vec<_> = self.resolve(ast).map(|entity| {
            self.implementors
                .iter()
                .flat_map(|implementor| implementor.get_implements(&entity))
                .collect()
        })?;

        Ok(quote! {
            #(#implements)*
        })
    }
}
