use crate::err::{MacroError, ViewResolveError, YukinoError};
use crate::interface::EntityView;
use proc_macro2::TokenStream;
use quote::quote;
use std::marker::PhantomData;
use syn::spanned::Spanned;
use syn::{parse2, ExprClosure, Pat};

pub type MacroResult<T> = Result<T, MacroError>;

pub struct ViewResolver<View: EntityView> {
    _marker: PhantomData<View>,
}

pub struct PreviousView {}

impl<View: EntityView> ViewResolver<View> {
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

    pub fn resolve_second_parameter(_pat: &Pat) -> MacroResult<PreviousView> {
        Ok(PreviousView {})
    }
}
