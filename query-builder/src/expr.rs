use crate::{DatabaseValue, FunctionCall, Ident};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ExprBox = Box<Expr>;

pub trait ExprNode {
    fn apply<V: ExprVisitor>(&self, visitor: &mut V);

    fn apply_mut<V: ExprMutVisitor>(&mut self, visitor: &mut V);
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

#[derive(Clone, Debug)]
pub enum Expr {
    Ident(Ident),
    Lit(DatabaseValue),
    FunctionCall(FunctionCall),
    BitInverse(ExprBox),
    BitXor(ExprBox, ExprBox),
    Mul(ExprBox, ExprBox),
    Div(ExprBox, ExprBox),
    Rem(ExprBox, ExprBox),
    Add(ExprBox, ExprBox),
    Sub(ExprBox, ExprBox),
    LeftShift(ExprBox, ExprBox),
    RightShift(ExprBox, ExprBox),
    BitAnd(ExprBox, ExprBox),
    BitOr(ExprBox, ExprBox),
    Bte(ExprBox, ExprBox),
    Lte(ExprBox, ExprBox),
    Neq(ExprBox, ExprBox),
    Bt(ExprBox, ExprBox),
    Lt(ExprBox, ExprBox),
    Eq(ExprBox, ExprBox),
    Not(ExprBox),
    And(ExprBox, ExprBox),
    Or(ExprBox, ExprBox),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expr::Ident(i) => i.fmt(f),
            Expr::Lit(l) => l.fmt(f),
            Expr::FunctionCall(c) => c.fmt(f),
            Expr::BitInverse(e) => write!(f, "~{}", e),
            Expr::BitXor(l, r) => write!(f, "{} ^ {}", l, r),
            Expr::Mul(l, r) => write!(f, "{} * {}", l, r),
            Expr::Div(l, r) => write!(f, "{} / {}", l, r),
            Expr::Rem(l, r) => write!(f, "{} % {}", l, r),
            Expr::Add(l, r) => write!(f, "{} + {}", l, r),
            Expr::Sub(l, r) => write!(f, "{} - {}", l, r),
            Expr::LeftShift(l, r) => write!(f, "{} << {}", l, r),
            Expr::RightShift(l, r) => write!(f, "{} >> {}", l, r),
            Expr::BitAnd(l, r) => write!(f, "{} & {}", l, r),
            Expr::BitOr(l, r) => write!(f, "{} | {}", l, r),
            Expr::Bte(l, r) => write!(f, "{} >= {}", l, r),
            Expr::Lte(l, r) => write!(f, "{} <= {}", l, r),
            Expr::Neq(l, r) => write!(f, "{} != {}", l, r),
            Expr::Bt(l, r) => write!(f, "{} > {}", l, r),
            Expr::Lt(l, r) => write!(f, "{} < {}", l, r),
            Expr::Eq(l, r) => write!(f, "{} == {}", l, r),
            Expr::Not(e) => write!(f, "!{}", e),
            Expr::And(l, r) => write!(f, "{} AND {}", l, r),
            Expr::Or(l, r) => write!(f, "{} OR {}", l, r),
        }
    }
}

impl ExprNode for Expr {
    fn apply<V: ExprVisitor>(&self, visitor: &mut V) {
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

    fn apply_mut<V: ExprMutVisitor>(&mut self, visitor: &mut V) {
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
