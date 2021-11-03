use crate::db::ty::{DatabaseType, DatabaseValue};
use crate::query::{FunctionCall, Ident};
use std::fmt::{Display, Formatter, Result as FmtResult};

type ExprBox = Box<Expr>;

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
    Xor(ExprBox, ExprBox),
    Or(ExprBox, ExprBox),
}

impl Expr {
    pub fn ty() -> DatabaseType {
        todo!()
    }
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
            Expr::Bt(l, r) => write!(f, "{} >{}", l, r),
            Expr::Lt(l, r) => write!(f, "{} < {}", l, r),
            Expr::Eq(l, r) => write!(f, "{} = {}", l, r),
            Expr::Not(e) => write!(f, "!{}", e),
            Expr::And(l, r) => write!(f, "{} AND {}", l, r),
            Expr::Xor(l, r) => write!(f, "{} XOR {}", l, r),
            Expr::Or(l, r) => write!(f, "{} OR {}", l, r),
        }
    }
}
