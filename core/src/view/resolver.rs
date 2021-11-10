use crate::err::{MacroError, ViewResolveError, YukinoError};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
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

        let (first_param_ident, second_param) = if item_closure.inputs.len() == 2 {
            let mut param_iter = item_closure.inputs.iter();
            Ok((
                if let Some(Pat::Ident(ident)) = param_iter.next() {
                    Self::unwrap_ident(ident)
                } else {
                    Err(ViewResolveError::UnexpectedParamPatternType(
                        "ident".to_string(),
                    ).as_macro_error(item_closure.inputs.first().unwrap().span()))
                }?,
                param_iter.next().unwrap(),
            ))
        } else {
            Err(
                ViewResolveError::UnexpectedCalculationParamCount(item_closure.inputs.len())
                    .as_macro_error(item_closure.span()),
            )
        }?;

        let previous_view = Self::unwrap_pre_view(second_param)?;

        let second_param = previous_view
            .input
            .map_or(quote! {_}, |ident| Pat::Ident(ident).to_token_stream());

        Ok(quote! {
            |#first_param_ident, #second_param| {
            }
        })
    }

    fn unwrap_ident(ident: &PatIdent) -> MacroResult<Ident> {
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

    fn unwrap_tuple(
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
                            let ident = Self::unwrap_ident(pat_ident)?;
                            let unwrap = quote! {
                                let #ident = #current
                            };

                            Ok((vec![ident], vec![unwrap]))
                        }
                        Pat::Tuple(pat_tuple) => Self::unwrap_tuple(pat_tuple, current),
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

    fn unwrap_pre_view(pat: &Pat) -> MacroResult<PreviousView> {
        match pat {
            Pat::Ident(ident) => Self::unwrap_ident(ident).map(|i| PreviousView {
                input: Some(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: i.clone(),
                    subpat: None,
                }),
                idents: vec![i],
                unwrap: TokenStream::new(),
            }),
            Pat::Tuple(tuple) => {
                let input = PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: format_ident!("__v"),
                    subpat: None,
                };
                let (idents, unwraps) = Self::unwrap_tuple(
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


#[test]
fn test_unwrap_preview() {
    use syn::parse_quote;
    let pat1 = parse_quote! {
        _
    };

    assert!(
        ViewResolver::unwrap_pre_view(&pat1).unwrap().idents.is_empty()
    );

    let pat2 = parse_quote! {
        v
    };

    assert_eq!(
        ViewResolver::unwrap_pre_view(&pat2).unwrap().idents,
        vec![format_ident!("v")]
    );

    let pat2 = parse_quote! {
        ((a, _), ((b, c), d))
    };

    assert_eq!(
        ViewResolver::unwrap_pre_view(&pat2).unwrap().idents,
        vec![
            format_ident!("a"),
            format_ident!("b"),
            format_ident!("c"),
            format_ident!("d"),
        ]
    );
}
