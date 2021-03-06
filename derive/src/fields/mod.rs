use std::any::type_name;

use heck::SnakeCase;
use quote::ToTokens;
use syn::{
    Error, Field, GenericArgument, Lit, Meta, parse_quote, parse_str, PathArguments, Result,
    ReturnType, Type,
};

pub use basic::*;
pub use datetime::*;
pub use decimal::*;

use crate::resolved::ResolvedField;

mod basic;
mod datetime;
mod decimal;

pub trait FieldResolver {
    fn can_resolve(&self, field: &Field) -> bool;

    fn resolve_field(&self, field: &Field) -> Result<ResolvedField>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TypeMatchResult {
    Match,
    Mismatch,
    InOption,
}

fn match_ty(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Path(a), Type::Path(b)) => {
            if a.path.segments.len() != b.path.segments.len() {
                return false;
            }
            a.path
                .segments
                .iter()
                .zip(b.path.segments.iter())
                .all(|(a, b)| {
                    a.ident == b.ident
                        && match (&a.arguments, &b.arguments) {
                            (PathArguments::None, PathArguments::None) => true,
                            (
                                PathArguments::AngleBracketed(l),
                                PathArguments::AngleBracketed(r),
                            ) => l.args.iter().zip(r.args.iter()).all(|(a, b)| match (a, b) {
                                (GenericArgument::Type(l_ty), GenericArgument::Type(r_ty)) => {
                                    match_ty(l_ty, r_ty)
                                }
                                (l_arg, r_arg) => {
                                    l_arg.to_token_stream().to_string()
                                        == r_arg.to_token_stream().to_string()
                                }
                            }),
                            (
                                PathArguments::Parenthesized(l_paren),
                                PathArguments::Parenthesized(r_paren),
                            ) => {
                                l_paren
                                    .inputs
                                    .iter()
                                    .zip(r_paren.inputs.iter())
                                    .all(|(a, b)| match_ty(a, b))
                                    && match (&l_paren.output, &r_paren.output) {
                                        (ReturnType::Default, ReturnType::Default) => true,
                                        (ReturnType::Type(_, l_ty), ReturnType::Type(_, r_ty)) => {
                                            match_ty(l_ty, r_ty)
                                        }
                                        _ => false,
                                    }
                            }
                            _ => false,
                        }
                })
        }
        (l, r) => l.to_token_stream().to_string() == r.to_token_stream().to_string(),
    }
}

fn match_optional_ty_by_param(target: &Type, input: &Type) -> TypeMatchResult {
    let target_option = parse_quote! {
        Option<#target>
    };

    if match_ty(input, target) {
        TypeMatchResult::Match
    } else if match_ty(input, &target_option) {
        TypeMatchResult::InOption
    } else {
        TypeMatchResult::Mismatch
    }
}

fn match_optional_ty<T>(input: &Type) -> TypeMatchResult {
    let target = parse_str(type_name::<T>()).unwrap();
    match_optional_ty_by_param(&target, input)
}

fn parse_field_name(field: &Field) -> Result<String> {
    field
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("name"))
        .map(|attr| match attr.parse_meta()? {
            Meta::NameValue(v) => match v.lit {
                Lit::Str(s) => Ok(s.value()),
                _ => Err(Error::new_spanned(v, "`name` attribute must be a str")),
            },
            _ => Err(Error::new_spanned(
                attr,
                "`name` attribute must be a named value",
            )),
        })
        .unwrap_or_else(|| Ok(field.ident.as_ref().unwrap().to_string().to_snake_case()))
}

#[test]
fn test_ty_match() {
    let ty1 = parse_quote! {
        u64
    };
    assert_eq!(match_optional_ty::<u64>(&ty1), TypeMatchResult::Match);
    let ty2 = parse_quote! {
        Option<u64>
    };
    assert_eq!(match_optional_ty::<u64>(&ty2), TypeMatchResult::InOption);
    assert_eq!(
        match_optional_ty_by_param(&ty2, &ty2),
        TypeMatchResult::Match
    );
    assert_eq!(match_optional_ty::<u32>(&ty2), TypeMatchResult::Mismatch);
    assert_eq!(
        match_optional_ty::<Option<u32>>(&ty2),
        TypeMatchResult::Mismatch
    );
}
