use crate::err::{MacroError, ViewResolveError, YukinoError};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse2, ExprClosure, Pat, PatIdent, PatTuple};

pub type MacroResult<T> = Result<T, MacroError>;

pub struct ViewResolver {}

pub struct PreviousView {
    pub input: Option<PatIdent>,
    pub idents: Vec<Ident>,
    pub unwrap: TokenStream,
}

impl ViewResolver {
    pub fn resolve(token_stream: TokenStream) -> MacroResult<TokenStream> {
        let item_closure: ExprClosure = parse2(token_stream).map_err(|e| MacroError {
            msg: e.to_string(),
            pos: e.span(),
        })?;

        let ((_entity_mut, _first_param_ident), second_param) = if item_closure.inputs.len() == 2 {
            let mut param_iter = item_closure.inputs.iter();
            Ok((
                if let Some(Pat::Ident(ident)) = param_iter.next() {
                    if ident.by_ref.is_some() {
                        Err(ViewResolveError::RefIsInvalid)
                    } else if ident.subpat.is_some() {
                        Err(ViewResolveError::SubPatternIsInvalid)
                    } else {
                        Ok((ident.mutability.is_some(), &ident.ident))
                    }
                } else {
                    Err(ViewResolveError::UnexpectedParamPatternType(
                        "ident".to_string(),
                    ))
                }
                    .map_err(|e| e.as_macro_error(item_closure.inputs.first().unwrap().span()))?,
                param_iter.next().unwrap(),
            ))
        } else {
            Err(
                ViewResolveError::UnexpectedCalculationParamCount(item_closure.inputs.len())
                    .as_macro_error(item_closure.span()),
            )
        }?;

        let _previous_view = Self::resolve_second_parameter(second_param)?;

        Ok(quote! {})
    }

    fn resolve_ident(ident: &PatIdent) -> MacroResult<Ident> {
        if ident.by_ref.is_some() {
            Err(ViewResolveError::RefIsInvalid.as_macro_error(ident.by_ref.unwrap().span()))
        } else if ident.subpat.is_some() {
            Err(ViewResolveError::SubPatternIsInvalid
                .as_macro_error(ident.subpat.as_ref().map(|(_, p)| p.span()).unwrap()))
        } else if ident.mutability.is_some() {
            Err(ViewResolveError::MutableIsInvalid.as_macro_error(ident.mutability.unwrap().span()))
        } else {
            Ok(ident.ident.clone())
        }
    }

    fn resolve_tuple(
        tuple: &PatTuple,
        parent: TokenStream,
    ) -> MacroResult<(Vec<Ident>, Vec<TokenStream>)> {
        if tuple.elems.len() == 2 {
            let (idents, unwrap_token_streams): (Vec<_>, Vec<_>) = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(index, ele)| {
                    let current = quote! {
                        #parent.#index
                    };
                    match ele {
                        Pat::Ident(pat_ident) => {
                            let ident = Self::resolve_ident(pat_ident)?;
                            let unwrap = quote! {
                                let #ident = #current
                            };

                            Ok((vec![ident], vec![unwrap]))
                        }
                        Pat::Tuple(pat_tuple) => Self::resolve_tuple(pat_tuple, current),
                        Pat::Wild(_) => Ok((vec![], vec![])),
                        _ => Err(ViewResolveError::CannotUnwrap.as_macro_error(ele.span())),
                    }
                })
                .collect::<MacroResult<Vec<_>>>()?
                .into_iter()
                .unzip();
            Ok((
                idents.into_iter().flatten().collect(),
                unwrap_token_streams.into_iter().flatten().collect(),
            ))
        } else {
            Err(ViewResolveError::NotTwoElementsTuple.as_macro_error(tuple.span()))
        }
    }

    fn resolve_second_parameter(pat: &Pat) -> MacroResult<PreviousView> {
        match pat {
            Pat::Ident(ident) => Self::resolve_ident(ident).map(|i| PreviousView {
                input: Some(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: i.clone(),
                    subpat: None,
                }),
                idents: vec![i],
                unwrap: Default::default(),
            }),
            Pat::Tuple(tuple) => {
                let input = PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: format_ident!("__v"),
                    subpat: None,
                };
                let (idents, unwraps) = Self::resolve_tuple(
                    tuple,
                    quote! {
                        #input
                    },
                )?;

                Ok(PreviousView {
                    input: Some(input),
                    idents,
                    unwrap: quote! {
                        #(#unwraps;)*
                    },
                })
            }
            Pat::Wild(_) => Ok(PreviousView {
                input: None,
                idents: vec![],
                unwrap: TokenStream::new(),
            }),
            _ => Err(ViewResolveError::CannotUnwrap.as_macro_error(pat.span())),
        }
    }
}
