use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, parse_quote, parse_str, Result, Type};

use interface::{ColumnDefinition, DatabaseType, FieldDefinition};

use crate::fields::{FieldResolver, match_optional_ty, match_optional_ty_by_param, parse_field_name, TypeMatchResult};
use crate::resolved::ResolvedField;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FieldType {
    Bool,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    Float,
    Double,
    String,
}

pub struct BasicFieldResolver;

impl FieldResolver for BasicFieldResolver {
    fn can_resolve(&self, field: &Field) -> bool {
        FieldType::from_ty(&field.ty).is_some()
    }

    fn resolve_field(&self, field: &Field) -> Result<ResolvedField> {
        let (ty, optional) = FieldType::from_ty(&field.ty).unwrap();
        let column_name = parse_field_name(field)?;

        let auto_increment = field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("auto_increment"));
        let primary_key = field.attrs.iter().any(|attr| attr.path.is_ident("id"));
        Ok(ResolvedField {
            name: field.ident.clone().unwrap(),
            ty: ty.field_ty(optional),
            view_construct: ty.view_construct(&column_name),
            view_ty: ty.view_ty(optional),
            view_full_path: ty.view_path(optional),
            vertical_ty: ty.vertical_view_ty(optional),
            vertical_full_path: ty.vertical_view_path(optional),
            tag_list: ty.tags(optional),
            converter_ty: ty.converter_ty(optional),
            field_marker: format_ident!("{}", column_name),
            definition: FieldDefinition {
                name: field.ident.as_ref().unwrap().to_string(),
                columns: vec![ColumnDefinition {
                    name: column_name.clone(),
                    ty: (&ty).into(),
                    optional,
                    auto_increment,
                }],
                identity_column: column_name,
                primary_key,
            },
            converter_value_count: 1,
        })
    }
}

impl FieldType {
    pub fn from_ty(ty: &Type) -> Option<(Self, bool)> {
        let branch: [(FieldType, Box<dyn Fn() -> TypeMatchResult>); 10] = [
            (FieldType::Bool, Box::new(|| match_optional_ty::<bool>(ty))),
            (FieldType::Short, Box::new(|| match_optional_ty::<i16>(ty))),
            (
                FieldType::UnsignedShort,
                Box::new(|| match_optional_ty::<u16>(ty)),
            ),
            (FieldType::Int, Box::new(|| match_optional_ty::<i32>(ty))),
            (
                FieldType::UnsignedInt,
                Box::new(|| match_optional_ty::<u32>(ty)),
            ),
            (FieldType::Long, Box::new(|| match_optional_ty::<i64>(ty))),
            (
                FieldType::UnsignedLong,
                Box::new(|| match_optional_ty::<u64>(ty)),
            ),
            (FieldType::Float, Box::new(|| match_optional_ty::<f32>(ty))),
            (FieldType::Double, Box::new(|| match_optional_ty::<f64>(ty))),
            (
                FieldType::String,
                Box::new(|| {
                    let str = parse_quote!(
                        String
                    );
                    match_optional_ty_by_param(&str, ty)
                }),
            ),
        ];
        branch
            .iter()
            .map(|(field_type, f)| match (*f)() {
                TypeMatchResult::Match => Some((*field_type, false)),
                TypeMatchResult::InOption => Some((*field_type, true)),
                TypeMatchResult::Mismatch => None,
            })
            .find(|r| r.is_some())
            .flatten()
    }

    pub fn field_ty(&self, optional: bool) -> TokenStream {
        let inside: TokenStream = parse_str(match self {
            FieldType::Bool => "bool",
            FieldType::Short => "i16",
            FieldType::UnsignedShort => "u16",
            FieldType::Int => "i32",
            FieldType::UnsignedInt => "u32",
            FieldType::Long => "i64",
            FieldType::UnsignedLong => "u64",
            FieldType::Float => "f32",
            FieldType::Double => "f64",
            FieldType::String => "String",
        })
        .unwrap();

        if optional {
            quote! {
                Option<#inside>
            }
        } else {
            inside
        }
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
        let ty = self.field_ty(optional);
        quote! {
            yukino::view::ExprViewBox<#ty>
        }
    }

    pub fn view_path(&self, optional: bool) -> TokenStream {
        let ty = self.field_ty(optional);

        quote! {
            yukino::view::SingleExprView::<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn vertical_view_ty(&self, optional: bool) -> TokenStream {
        let ty = self.field_ty(optional);

        quote! {
            yukino::view::VerticalExprView<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn vertical_view_path(&self, optional: bool) -> TokenStream {
        let ty = self.field_ty(optional);

        quote! {
            yukino::view::VerticalExprView::<#ty, yukino::view::TagsOfValueView<#ty>>
        }
    }

    pub fn tags(&self, optional: bool) -> TokenStream {
        let ty = self.field_ty(optional);

        quote! {
            yukino::view::TagsOfValueView<#ty>
        }
    }

    pub fn converter_ty(&self, optional: bool) -> TokenStream {
        let prefix = if optional { "Optional" } else { "" };
        let name = match self {
            FieldType::Bool => format_ident!("{}BoolConverter", prefix),
            FieldType::Short => format_ident!("{}ShortConverter", prefix),
            FieldType::UnsignedShort => format_ident!("{}UnsignedShortConverter", prefix),
            FieldType::Int => format_ident!("{}IntConverter", prefix),
            FieldType::UnsignedInt => format_ident!("{}UnsignedIntConverter", prefix),
            FieldType::Long => format_ident!("{}LongConverter", prefix),
            FieldType::UnsignedLong => format_ident!("{}UnsignedLongConverter", prefix),
            FieldType::Float => format_ident!("{}FloatConverter", prefix),
            FieldType::Double => format_ident!("{}DoubleConverter", prefix),
            FieldType::String => format_ident!("{}StringConverter", prefix),
        };

        quote! {
            yukino::converter::#name
        }
    }
}

impl From<&FieldType> for DatabaseType {
    fn from(ty: &FieldType) -> Self {
        match ty {
            FieldType::Bool => DatabaseType::Bool,
            FieldType::Short => DatabaseType::SmallInteger,
            FieldType::UnsignedShort => DatabaseType::UnsignedSmallInteger,
            FieldType::Int => DatabaseType::Integer,
            FieldType::UnsignedInt => DatabaseType::UnsignedInteger,
            FieldType::Long => DatabaseType::BigInteger,
            FieldType::UnsignedLong => DatabaseType::UnsignedBigInteger,
            FieldType::Float => DatabaseType::Float,
            FieldType::Double => DatabaseType::Double,
            FieldType::String => DatabaseType::String,
        }
    }
}
