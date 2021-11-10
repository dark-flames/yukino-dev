use crate::closure::context::{Context, VariableType};
use crate::view::MacroResult;
use core::option::Option;
use core::result::Result::Ok;
use proc_macro2::Ident;
use std::collections::HashMap;

pub struct BlockContext<'t> {
    base: &'t dyn Context,
    current: HashMap<Ident, VariableType>,
}

impl<'t> Context for BlockContext<'t> {
    fn append(&mut self, ident: Ident, ty: VariableType) -> MacroResult<()> {
        self.current.insert(ident, ty);

        Ok(())
    }

    fn find(&self, ident: &Ident) -> Option<VariableType> {
        self.current
            .get(ident)
            .copied()
            .or_else(|| self.base.find(ident))
    }
}

impl<'t> BlockContext<'t> {
    pub fn create(base: &'t dyn Context, current: HashMap<Ident, VariableType>) -> Self {
        BlockContext { base, current }
    }
}
