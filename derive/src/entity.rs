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
        implementors: Vec<Box<dyn Implementor>>) -> Self {
        EntityResolver {
            field_resolvers,
            implementors
        }
    }

    pub fn resolve(&self, ast: &ItemStruct) -> Result<ResolvedEntity> {
        let entity_name = ast.ident.clone();
        let table_name = ast.attrs.iter().find(
            |attr| attr.path.is_ident("name")
        ).map(|attr| {
            attr.parse_args::<LitStr>().map(|lit| lit.value())
        }).unwrap_or_else(|| {
            Ok(ast.ident.to_string().to_snake_case())
        })?;

        let associations = ast.attrs.iter().filter(
            |attr| attr.path.is_ident("belongs_to")
        ).map(|attr| {
            match attr.parse_meta()? {
                Meta::List(l) => {
                    let mut iter = l.nested.iter();
                    let ref_entity_path = iter.next().ok_or_else(
                        || Error::new_spanned(
                            attr,
                            "Expected referenced entity path"
                        )
                    ).and_then(
                        |meta| {
                            match meta {
                                NestedMeta::Meta(Meta::Path(p)) => {
                                    Ok(p.clone())
                                }
                                _ => {
                                    Err(Error::new_spanned(
                                        attr,
                                        "Expected referenced entity path"
                                    ))
                                }
                            }
                        }
                    )?;

                    let foreign_key = iter.next().ok_or_else(
                        || Error::new_spanned(
                            attr,
                            "Expected foreign key"
                        )
                    ).and_then(
                        |meta| {
                            match meta {
                                NestedMeta::Meta(Meta::Path(p)) => {
                                    Ok(p.get_ident().ok_or_else(
                                        || Error::new_spanned(
                                            attr,
                                            "Expected foreign key"
                                        )
                                    )?.clone())
                                }
                                _ => {
                                    Err(Error::new_spanned(
                                        attr,
                                        "Expected foreign key"
                                    ))
                                }
                            }
                        }
                    )?;
                    Ok(ResolvedAssociation {
                        ref_entity_path,
                        foreign_key
                    })
                }
                _ => {
                    Err(Error::new_spanned(
                        attr,
                        "`belongs_to` attribute must be a list"
                    ))
                }
            }
        }).collect::<Result<Vec<_>>>()?;

        let fields = if let Fields::Named(name_fields) = &ast.fields {
            name_fields.named.iter().map(
                |field| self.field_resolvers.iter().find(
                    |resolver| resolver.can_resolve(field)
                ).ok_or_else(
                    || Error::new_spanned(
                        field,
                        "Cannot find a field resolver for this field"
                    )
                ).and_then(
                    |resolver| resolver.resolve_field(field)
                )
            ).collect::<Result<Vec<_>>>()
        } else {
            return Err(Error::new_spanned(
                ast,
                "Expected named fields"
            ))
        }?;

        Ok(ResolvedEntity {
            table_name,
            view_name: format_ident!("{}View", entity_name),
            vertical_name: format_ident!("Vertical{}View", entity_name),
            converter_name: format_ident!("{}Converter", entity_name),
            entity_name,
            fields,
            associations
        })
    }

    pub fn get_implements(&self, ast: &ItemStruct) -> Result<TokenStream> {
        let implements: Vec<_> = self.resolve(ast).map(
            |entity| self.implementors.iter().flat_map(
                |implementor| implementor.get_implements(&entity)
            ).collect()
        )?;

        Ok(quote! {
            #(#implements)*
        })
    }
}