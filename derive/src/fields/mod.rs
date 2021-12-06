use std::any::type_name;

use quote::ToTokens;
use syn::{Field, GenericArgument, parse_quote, parse_str, PathArguments, Result, ReturnType, Type};

pub use basic::*;

use crate::resolved::ResolvedField;

mod basic;

pub trait FieldResolver {
    fn can_resolve(&self, field: &Field) -> bool;

    fn resolve_field(&self, field: &Field) -> Result<ResolvedField>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypeMatchResult {
    Match,
    Mismatch,
    InOption
}

pub fn match_ty(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Path(a), Type::Path(b)) => {
            if a.path.segments.len() != b.path.segments.len() {
                return false;
            }
            a.path.segments.iter()
                .zip(b.path.segments.iter())
                .all(|(a, b)| {
                    a.ident == b.ident && match (&a.arguments, &b.arguments) {
                        (PathArguments::None, PathArguments::None) => true,
                        (PathArguments::AngleBracketed(l), PathArguments::AngleBracketed(r)) => {
                            l.args.iter()
                                .zip(r.args.iter())
                                .all(|(a, b)| {
                                    match (a, b) {
                                        (GenericArgument::Type(l_ty), GenericArgument::Type(r_ty)) => {
                                            match_ty(l_ty, r_ty)
                                        },
                                        (l_arg, r_arg) =>
                                            l_arg.to_token_stream().to_string() == r_arg.to_token_stream().to_string()
                                    }
                                })
                        }
                        (PathArguments::Parenthesized(l_paren), PathArguments::Parenthesized(r_paren)) => {
                            l_paren.inputs.iter()
                                .zip(r_paren.inputs.iter())
                                .all(|(a, b)| match_ty(a, b))
                                && match (&l_paren.output, &r_paren.output) {
                                (ReturnType::Default, ReturnType::Default) => true,
                                (ReturnType::Type(_, l_ty), ReturnType::Type(_, r_ty)) => match_ty(l_ty, r_ty),
                                _ => false
                            }
                        }
                        _ => false
                    }
                })
        },
        (l, r) => l.to_token_stream().to_string() == r.to_token_stream().to_string()
    }
}

pub fn match_optional_ty_by_param(target: &Type, input: &Type) -> TypeMatchResult {
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

pub fn match_optional_ty<T>(input: &Type) -> TypeMatchResult {
    let target = parse_str(type_name::<T>()).unwrap();
    match_optional_ty_by_param(
        &target,
        input
    )
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
    assert_eq!(match_optional_ty_by_param(&ty2, &ty2), TypeMatchResult::Match);
    assert_eq!(match_optional_ty::<u32>(&ty2), TypeMatchResult::Mismatch);
    assert_eq!(match_optional_ty::<Option<u32>>(&ty2), TypeMatchResult::Mismatch);
}