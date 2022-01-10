use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Field, parse_quote, Type};

use interface::{ColumnDefinition, DatabaseType, FieldDefinition};

use crate::fields::{FieldResolver, match_optional_ty_by_param, parse_field_name, TypeMatchResult};
use crate::resolved::ResolvedField;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DateTimeType {
    Date,
    Time,
    DateTime,
}

impl DateTimeType {
    pub fn ty(&self) -> Type {
        match self {
            DateTimeType::Date => parse_quote! {
                sqlx::types::time::Date
            },
            DateTimeType::Time => parse_quote! {
                sqlx::types::time::Time
            },
            DateTimeType::DateTime => parse_quote! {
                sqlx::types::time::PrimitiveDateTime
            },
        }
    }

    pub fn full_ty(&self, optional: bool) -> TokenStream {
        let full = self.ty().to_token_stream();

        if optional {
            quote! {
                Option<#full>
            }
        } else {
            full
        }
    }

    pub fn database_ty(&self) -> DatabaseType {
        match self {
            DateTimeType::Date => DatabaseType::Date,
            DateTimeType::Time => DatabaseType::Time,
            DateTimeType::DateTime => DatabaseType::DateTime,
        }
    }

    pub fn from_ty(ty: &Type) -> Option<(Self, bool)> {
        let tys: [DateTimeType; 3] = [
            DateTimeType::Date,
            DateTimeType::Time,
            DateTimeType::DateTime,
        ];

        tys.iter()
            .map(|datetime_ty| {
                let target = datetime_ty.ty();
                match match_optional_ty_by_param(&target, ty) {
                    TypeMatchResult::Match => Some((*datetime_ty, false)),
                    TypeMatchResult::InOption => Some((*datetime_ty, true)),
                    TypeMatchResult::Mismatch => None,
                }
            })
            .find(|r| r.is_some())
            .flatten()
    }

    pub fn view_construct(&self, column: &str) -> TokenStream {
        quote! {
            {
                use yukino::view::AnyTagExprView;
                yukino::view::SingleExprView::from_exprs_with_tags
                (yukino::generic_array::arr![yukino::query_builder::Expr;
                    alias.create_ident_expr(#column)
                ])
            }
        }
    }

    pub fn view_ty(&self, optional: bool) -> TokenStream {
        let ty = self.full_ty(optional);
        quote! {
            yukino::view::ExprViewBox<#ty>
        }
    }

    pub fn view_path(&self, optional: bool) -> TokenStream {
        let ty = self.full_ty(optional);

        quote! {
            yukino::view::SingleExprView::<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn vertical_view_ty(&self, optional: bool) -> TokenStream {
        let ty = self.full_ty(optional);

        quote! {
            yukino::view::VerticalExprView<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn vertical_view_path(&self, optional: bool) -> TokenStream {
        let ty = self.full_ty(optional);

        quote! {
            yukino::view::VerticalExprView::<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn tags(&self, optional: bool) -> TokenStream {
        let ty = self.full_ty(optional);

        quote! {
            yukino::view::TagsOfValueView<#ty>
        }
    }

    pub fn converter_ty(&self, optional: bool) -> TokenStream {
        let prefix = if optional { "Optional" } else { "" };
        let name = match self {
            DateTimeType::Date => format_ident!("{}DateConverter", prefix),
            DateTimeType::Time => format_ident!("{}TimeConverter", prefix),
            DateTimeType::DateTime => format_ident!("{}DateTimeConverter", prefix),
        };

        quote! {
            yukino::converter::#name
        }
    }
}

pub struct DateTimeFieldResolver;

impl FieldResolver for DateTimeFieldResolver {
    fn can_resolve(&self, field: &Field) -> bool {
        DateTimeType::from_ty(&field.ty).is_some()
    }

    fn resolve_field(&self, field: &Field) -> syn::Result<ResolvedField> {
        let (field_ty, optional) = DateTimeType::from_ty(&field.ty).unwrap();
        let column_name = parse_field_name(field)?;

        Ok(ResolvedField {
            name: field.ident.clone().unwrap(),
            definition: FieldDefinition {
                name: field.ident.as_ref().unwrap().to_string(),
                columns: vec![ColumnDefinition {
                    name: column_name.clone(),
                    ty: field_ty.database_ty(),
                    optional,
                    auto_increment: false,
                }],
                identity_column: column_name.clone(),
                primary_key: false,
            },
            ty: field_ty.full_ty(optional),
            view_construct: field_ty.view_construct(&column_name),
            view_ty: field_ty.view_ty(optional),
            view_full_path: field_ty.view_path(optional),
            vertical_ty: field_ty.vertical_view_ty(optional),
            vertical_full_path: field_ty.vertical_view_path(optional),
            tag_list: field_ty.tags(optional),
            converter_ty: field_ty.converter_ty(optional),
            converter_value_count: 1,
            field_marker: format_ident!("{}", column_name),
        })
    }
}
