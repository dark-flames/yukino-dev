use crate::closure::context::{Context, VariableType};
use crate::err::{ViewResolveError, YukinoError};
use crate::view::MacroResult;
use proc_macro2::Ident;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct ParamsContext<'t> {
    base: Option<&'t dyn Context>,
    params: HashMap<Ident, VariableType>,
}

impl<'t> Context for ParamsContext<'t> {
    fn append(&mut self, ident: Ident, ty: VariableType) -> MacroResult<()> {
        let span = ident.span();
        if let Entry::Vacant(e) = self.params.entry(ident) {
            e.insert(ty);
            Ok(())
        } else {
            Err(ViewResolveError::IdentConflict.as_macro_error(span))
        }
    }

    fn find(&self, ident: &Ident) -> Option<VariableType> {
        self.params
            .get(ident)
            .copied()
            .or_else(|| self.base.and_then(|b| b.find(ident)))
    }
}

impl<'t> ParamsContext<'t> {
    pub fn create(base: Option<&'t dyn Context>, params: HashMap<Ident, VariableType>) -> Self {
        ParamsContext { base, params }
    }
}
