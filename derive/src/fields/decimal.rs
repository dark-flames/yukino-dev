use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Field, parse_quote, Result, Type};

use interface::{ColumnDefinition, DatabaseType, FieldDefinition};

use crate::fields::{FieldResolver, match_optional_ty_by_param, parse_field_name, TypeMatchResult};
use crate::resolved::ResolvedField;

fn match_decimal_ty(ty: &Type) -> Option<(TokenStream, bool)> {
    let full_decimal_ty: Type = parse_quote! {
        sqlx::types::Decimal
    };

    let result = match_optional_ty_by_param(&full_decimal_ty, ty);

    match result {
        TypeMatchResult::Match => Some((full_decimal_ty.to_token_stream(), false)),
        TypeMatchResult::Mismatch => None,
        TypeMatchResult::InOption => Some((
            quote! {
                Option<#full_decimal_ty>
            },
            true,
        )),
    }
}

pub struct DecimalFieldResolver;

impl FieldResolver for DecimalFieldResolver {
    fn can_resolve(&self, field: &Field) -> bool {
        match_decimal_ty(&field.ty).is_some()
    }

    fn resolve_field(&self, field: &Field) -> Result<ResolvedField> {
        let (full_ty, optional) = match_decimal_ty(&field.ty).unwrap();
        let column_name = parse_field_name(field)?;

        Ok(ResolvedField {
            name: field.ident.clone().unwrap(),
            definition: FieldDefinition {
                name: field.ident.as_ref().unwrap().to_string(),
                columns: vec![ColumnDefinition {
                    name: column_name.clone(),
                    ty: DatabaseType::Decimal,
                    optional,
                    auto_increment: false
                }],
                identity_column: column_name.clone(),
                primary_key: false,
            },
            ty: field.ty.clone().to_token_stream(),
            view_construct: quote! {
                {
                    use yukino::view::AnyTagExprView;
                    yukino::view::SingleExprView::from_exprs_with_tags(
                        yukino::generic_array::arr![yukino::query_builder::Expr;
                            alias.create_ident_expr(#column_name)
                        ]
                    )
                }
            },
            view_ty: quote! {
                yukino::view::ExprViewBox<#full_ty>
            },
            view_full_path: quote! {
                yukino::view::SingleExprView::<#full_ty, yukino::view::TagsOfValueView<#full_ty>>
            },
            vertical_ty: quote! {
                yukino::view::VerticalExprView<#full_ty, yukino::view::TagsOfValueView<#full_ty>>
            },
            vertical_full_path: quote! {
                yukino::view::VerticalExprView::<#full_ty, yukino::view::TagsOfValueView<#full_ty>>
            },
            tag_list: quote! {
                yukino::view::TagsOfValueView<#full_ty>
            },
            converter_ty: if optional {
                quote! {
                    yukino::converter::OptionalDecimalConverter
                }
            } else {
                quote! {
                    yukino::converter::DecimalConverter
                }
            },
            converter_value_count: 1,
            field_marker: format_ident!("{}", column_name),
            primary: false
        })
    }
}
