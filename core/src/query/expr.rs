use crate::db::ty::{DatabaseType, DatabaseValue};
use crate::query::{FunctionCall, Ident};

type ExprBox = Box<Expr>;

#[derive(Clone)]
pub enum Expr {
    Ident(Ident),
    Lit(DatabaseValue),
    FunctionCall(FunctionCall),
    BitReverse(ExprBox),
    BitXor(ExprBox, ExprBox),
    Multi(ExprBox, ExprBox),
    Div(ExprBox, ExprBox),
    Mod(ExprBox, ExprBox),
    Plus(ExprBox, ExprBox),
    Minus(ExprBox, ExprBox),
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
