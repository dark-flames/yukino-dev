use crate::closure::{BlockContext, Context, ParamsContext, VariableType};
use crate::err::{MacroError, ViewResolveError, YukinoError};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse2, Expr, ExprArray, ExprClosure, Pat, PatIdent, PatTuple};

pub type MacroResult<T> = Result<T, MacroError>;

pub struct ViewResolver;

impl ViewResolver {
    pub fn resolve(token_stream: TokenStream) -> MacroResult<TokenStream> {
        let item_closure: ExprClosure = parse2(token_stream).map_err(|e| MacroError {
            msg: e.to_string(),
            pos: e.span(),
        })?;

        let mut context = ParamsContext::default();

        let (first_param_ident, second_param) = if item_closure.inputs.len() == 2 {
            let mut param_iter = item_closure.inputs.iter();
            Ok((
                if let Some(Pat::Ident(ident)) = param_iter.next() {
                    Self::resolve_param_ident(&mut context, ident)
                } else {
                    Err(
                        ViewResolveError::UnexpectedParamPatternType("ident".to_string())
                            .as_macro_error(item_closure.inputs.first().unwrap().span()),
                    )
                }?,
                param_iter.next().unwrap(),
            ))
        } else {
            Err(
                ViewResolveError::UnexpectedCalculationParamCount(item_closure.inputs.len())
                    .as_macro_error(item_closure.span()),
            )
        }?;

        let (second_ident, unwrap) = Self::resolve_pre_view(&mut context, second_param)?;

        let second_param =
            second_ident.map_or(quote! {_}, |ident| Pat::Ident(ident).to_token_stream());

        let mut expr_context = context.subcontext();
        let expr = Self::resolve_expr(&mut expr_context, item_closure.body.as_ref())?;

        Ok(quote! {
            |#first_param_ident, #second_param| {
                #unwrap
                #expr
            }
        })
    }

    fn resolve_expr(context: &mut BlockContext, expr: &Expr) -> MacroResult<Expr> {
        match expr {
            Expr::Array(arr) => Ok(Expr::Array(ExprArray {
                attrs: arr.attrs.clone(),
                bracket_token: arr.bracket_token,
                elems: arr
                    .elems
                    .iter()
                    .map(|e| Self::resolve_expr(context, e))
                    .collect::<MacroResult<_>>()?,
            })),
            // Expr::Assign(_) => {}
            // Expr::AssignOp(_) => {}
            // Expr::Async(_) => {}
            // Expr::Await(_) => {}
            // Expr::Binary(_) => {}
            // Expr::Block(_) => {}
            // Expr::Box(_) => {}
            // Expr::Break(_) => {}
            // Expr::Call(_) => {}
            // Expr::Cast(_) => {}
            // Expr::Closure(_) => {}
            // Expr::Continue(_) => {}
            // Expr::Field(_) => {}
            // Expr::ForLoop(_) => {}
            // Expr::Group(_) => {}
            // Expr::If(_) => {}
            // Expr::Index(_) => {}
            // Expr::Let(_) => {}
            // Expr::Lit(_) => {}
            // Expr::Loop(_) => {}
            // Expr::Macro(_) => {}
            // Expr::Match(_) => {}
            // Expr::MethodCall(_) => {}
            // Expr::Paren(_) => {}
            // Expr::Path(_) => {}
            // Expr::Range(_) => {}
            // Expr::Reference(_) => {}
            // Expr::Repeat(_) => {}
            // Expr::Return(_) => {}
            // Expr::Struct(_) => {}
            // Expr::Try(_) => {}
            // Expr::TryBlock(_) => {}
            // Expr::Tuple(_) => {}
            // Expr::Type(_) => {}
            // Expr::Unary(_) => {}
            // Expr::Unsafe(_) => {}
            // Expr::Verbatim(_) => {}
            // Expr::While(_) => {}
            e => Ok(e.clone()),
        }
    }

    fn resolve_param_ident(context: &mut dyn Context, ident: &PatIdent) -> MacroResult<Ident> {
        if ident.by_ref.is_some() {
            Err(ViewResolveError::RefIsInvalid.as_macro_error(ident.by_ref.unwrap().span()))
        } else if ident.subpat.is_some() {
            Err(ViewResolveError::SubPatternIsInvalid
                .as_macro_error(ident.subpat.as_ref().map(|(_, p)| p.span()).unwrap()))
        } else if ident.mutability.is_some() {
            Err(ViewResolveError::MutableIsInvalid.as_macro_error(ident.mutability.unwrap().span()))
        } else {
            context.append(ident.ident.clone(), VariableType::View)?;
            Ok(ident.ident.clone())
        }
    }

    fn resolve_param_tuple(
        context: &mut dyn Context,
        tuple: &PatTuple,
        parent: TokenStream,
    ) -> MacroResult<Vec<TokenStream>> {
        if tuple.elems.len() == 2 {
            let unwrap_token_streams: Vec<_> = tuple
                .elems
                .iter()
                .enumerate()
                .map(|(index, ele)| {
                    let current = quote! {
                        #parent.#index
                    };
                    match ele {
                        Pat::Ident(pat_ident) => {
                            let ident = Self::resolve_param_ident(context, pat_ident)?;
                            let unwrap = quote! {
                                let #ident = #current
                            };

                            Ok(vec![unwrap])
                        }
                        Pat::Tuple(pat_tuple) => {
                            Self::resolve_param_tuple(context, pat_tuple, current)
                        }
                        Pat::Wild(_) => Ok(vec![]),
                        _ => Err(ViewResolveError::CannotUnwrap.as_macro_error(ele.span())),
                    }
                })
                .collect::<MacroResult<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect();
            Ok(unwrap_token_streams)
        } else {
            Err(ViewResolveError::NotTwoElementsTuple.as_macro_error(tuple.span()))
        }
    }

    fn resolve_pre_view(
        context: &mut dyn Context,
        pat: &Pat,
    ) -> MacroResult<(Option<PatIdent>, TokenStream)> {
        match pat {
            Pat::Ident(ident) => Self::resolve_param_ident(context, ident).map(|i| {
                (
                    Some(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: i,
                        subpat: None,
                    }),
                    TokenStream::new(),
                )
            }),
            Pat::Tuple(tuple) => {
                let input = PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: format_ident!("__v"),
                    subpat: None,
                };
                let unwraps = Self::resolve_param_tuple(
                    context,
                    tuple,
                    quote! {
                        #input
                    },
                )?;

                Ok((
                    Some(input),
                    quote! {
                        #(#unwraps;)*
                    },
                ))
            }
            Pat::Wild(_) => Ok((None, TokenStream::new())),
            _ => Err(ViewResolveError::CannotUnwrap.as_macro_error(pat.span())),
        }
    }
}
