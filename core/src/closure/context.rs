use crate::closure::expr::BlockContext;
use crate::closure::param::ParamsContext;
use crate::view::MacroResult;
use proc_macro2::Ident;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VariableType {
    Value,
    View,
}

pub trait Context {
    fn subcontext(&self) -> BlockContext
    where
        Self: Sized,
    {
        BlockContext::create(self, Default::default())
    }

    fn sub_closure_param_context(&self) -> ParamsContext
    where
        Self: Sized,
    {
        ParamsContext::create(Some(self), Default::default())
    }

    fn append(&mut self, ident: Ident, ty: VariableType) -> MacroResult<()>;

    fn find(&self, ident: &Ident) -> Option<VariableType>;
}
