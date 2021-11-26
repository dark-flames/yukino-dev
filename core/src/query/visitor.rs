use query_builder::{DatabaseValue, Expr, ExprBox, FunctionCall, Ident};

use crate::view::{ExprViewBoxWithTag, TagList, Value};

pub trait ExprNode {
    fn apply(&self, visitor: &mut dyn ExprVisitor);

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor);
}

pub trait ExprMutVisitor {
    fn visit_ident(&mut self, _node: &mut Ident) {}
    fn visit_lit(&mut self, _node: &mut DatabaseValue) {}
    fn visit_function_call(&mut self, _node: &mut FunctionCall) {}
    fn visit_bit_inverse(&mut self, _node: &mut ExprBox) {}
    fn visit_bit_xor(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_mul(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_div(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_rem(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_add(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_sub(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_left_shift(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_right_shift(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_bit_and(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_bit_or(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_bte(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_lte(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_neq(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_bt(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_lt(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_eq(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_not(&mut self, _node: &mut ExprBox) {}
    fn visit_and(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_or(&mut self, _l: &mut ExprBox, _r: &mut ExprBox) {}
    fn visit_expr(&mut self, _node: &mut Expr) {}
}

pub trait ExprVisitor {
    fn visit_ident(&mut self, _node: &Ident) {}
    fn visit_lit(&mut self, _node: &DatabaseValue) {}
    fn visit_function_call(&mut self, _node: &FunctionCall) {}
    fn visit_bit_inverse(&mut self, _node: &ExprBox) {}
    fn visit_bit_xor(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_mul(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_div(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_rem(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_add(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_sub(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_left_shift(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_right_shift(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_bit_and(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_bit_or(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_bte(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_lte(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_neq(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_bt(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_lt(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_eq(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_not(&mut self, _node: &ExprBox) {}
    fn visit_and(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_or(&mut self, _l: &ExprBox, _r: &ExprBox) {}
    fn visit_expr(&mut self, _node: &Expr) {}
}

impl ExprNode for Expr {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_expr(self);
        match self {
            Expr::Ident(ident) => ident.apply(visitor),
            Expr::Lit(lit) => lit.apply(visitor),
            Expr::FunctionCall(func_call) => func_call.apply(visitor),
            Expr::BitInverse(e) => {
                e.apply(visitor);
                visitor.visit_bit_inverse(e);
            }
            Expr::BitXor(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_bit_xor(l, r);
            }
            Expr::Mul(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_mul(l, r);
            }
            Expr::Div(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_div(l, r);
            }
            Expr::Rem(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_rem(l, r);
            }
            Expr::Add(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_add(l, r);
            }
            Expr::Sub(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_sub(l, r);
            }
            Expr::LeftShift(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_left_shift(l, r);
            }
            Expr::RightShift(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_right_shift(l, r);
            }
            Expr::BitAnd(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_bit_and(l, r);
            }
            Expr::BitOr(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_bit_or(l, r);
            }
            Expr::Bte(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_bte(l, r);
            }
            Expr::Lte(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_lte(l, r);
            }
            Expr::Neq(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_neq(l, r);
            }
            Expr::Bt(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_bit_xor(l, r);
            }
            Expr::Lt(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_lt(l, r);
            }
            Expr::Eq(l, r) => {
                visitor.visit_eq(l, r);
                l.apply(visitor);
                r.apply(visitor);
            }
            Expr::Not(e) => {
                e.apply(visitor);
                visitor.visit_not(e);
            }
            Expr::And(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_and(l, r);
            }
            Expr::Or(l, r) => {
                l.apply(visitor);
                r.apply(visitor);
                visitor.visit_or(l, r);
            }
        }
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_expr(self);
        match self {
            Expr::Ident(ident) => ident.apply_mut(visitor),
            Expr::Lit(lit) => lit.apply_mut(visitor),
            Expr::FunctionCall(func_call) => func_call.apply_mut(visitor),
            Expr::BitInverse(e) => {
                e.apply_mut(visitor);
                visitor.visit_bit_inverse(e);
            }
            Expr::BitXor(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_bit_xor(l, r);
            }
            Expr::Mul(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_mul(l, r);
            }
            Expr::Div(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_div(l, r);
            }
            Expr::Rem(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_rem(l, r);
            }
            Expr::Add(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_add(l, r);
            }
            Expr::Sub(l, r) => {
                visitor.visit_sub(l, r);
                l.apply_mut(visitor);
                r.apply_mut(visitor);
            }
            Expr::LeftShift(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_left_shift(l, r);
            }
            Expr::RightShift(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_right_shift(l, r);
            }
            Expr::BitAnd(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_bit_and(l, r);
            }
            Expr::BitOr(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_bit_or(l, r);
            }
            Expr::Bte(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_bte(l, r);
            }
            Expr::Lte(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_lte(l, r);
            }
            Expr::Neq(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_neq(l, r);
            }
            Expr::Bt(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_bit_xor(l, r);
            }
            Expr::Lt(l, r) => {
                visitor.visit_lt(l, r);
                l.apply_mut(visitor);
                r.apply_mut(visitor);
            }
            Expr::Eq(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_eq(l, r);
            }
            Expr::Not(e) => {
                visitor.visit_not(e);
                e.apply_mut(visitor);
            }
            Expr::And(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_and(l, r);
            }
            Expr::Or(l, r) => {
                l.apply_mut(visitor);
                r.apply_mut(visitor);
                visitor.visit_or(l, r);
            }
        }
    }
}

impl ExprNode for FunctionCall {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_function_call(self);
        self.params.iter().for_each(|p| p.apply(visitor))
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_function_call(self);
        self.params.iter_mut().for_each(|p| p.apply_mut(visitor))
    }
}

impl ExprNode for Ident {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_ident(self)
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_ident(self)
    }
}

impl ExprNode for DatabaseValue {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        visitor.visit_lit(self)
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        visitor.visit_lit(self)
    }
}

impl<T1: Value, T1Tag: TagList> ExprNode for ExprViewBoxWithTag<T1, T1Tag> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.as_ref().apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.as_mut().apply_mut(visitor)
    }
}

impl<T1: Value, T1Tag: TagList, T2: Value, T2Tag: TagList> ExprNode
    for (ExprViewBoxWithTag<T1, T1Tag>, ExprViewBoxWithTag<T2, T2Tag>)
{
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.0.apply(visitor);
        self.1.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.0.apply_mut(visitor);
        self.1.apply_mut(visitor);
    }
}
