use crate::{DatabaseValue, FunctionCall, Ident, SelectQuery};

pub type ExprBox = Box<Expr>;

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    Lit(DatabaseValue),
    FunctionCall(Box<dyn FunctionCall>),
    Subquery(SelectQuery),
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

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Ident(ident) => Expr::Ident(ident.clone()),
            Expr::Lit(lit) => Expr::Lit(lit.clone()),
            Expr::FunctionCall(func) => Expr::FunctionCall(func.boxed()),
            Expr::Subquery(subquery) => Expr::Subquery(subquery.clone()),
            Expr::BitInverse(expr) => Expr::BitInverse(expr.clone()),
            Expr::BitXor(expr1, expr2) => Expr::BitXor(expr1.clone(), expr2.clone()),
            Expr::Mul(expr1, expr2) => Expr::Mul(expr1.clone(), expr2.clone()),
            Expr::Div(expr1, expr2) => Expr::Div(expr1.clone(), expr2.clone()),
            Expr::Rem(expr1, expr2) => Expr::Rem(expr1.clone(), expr2.clone()),
            Expr::Add(expr1, expr2) => Expr::Add(expr1.clone(), expr2.clone()),
            Expr::Sub(expr1, expr2) => Expr::Sub(expr1.clone(), expr2.clone()),
            Expr::LeftShift(expr1, expr2) => Expr::LeftShift(expr1.clone(), expr2.clone()),
            Expr::RightShift(expr1, expr2) => Expr::RightShift(expr1.clone(), expr2.clone()),
            Expr::BitAnd(expr1, expr2) => Expr::BitAnd(expr1.clone(), expr2.clone()),
            Expr::BitOr(expr1, expr2) => Expr::BitOr(expr1.clone(), expr2.clone()),
            Expr::Bte(expr1, expr2) => Expr::Bte(expr1.clone(), expr2.clone()),
            Expr::Lte(expr1, expr2) => Expr::Lte(expr1.clone(), expr2.clone()),
            Expr::Neq(expr1, expr2) => Expr::Neq(expr1.clone(), expr2.clone()),
            Expr::Bt(expr1, expr2) => Expr::Bt(expr1.clone(), expr2.clone()),
            Expr::Lt(expr1, expr2) => Expr::Lt(expr1.clone(), expr2.clone()),
            Expr::Eq(expr1, expr2) => Expr::Eq(expr1.clone(), expr2.clone()),
            Expr::Not(expr) => Expr::Not(expr.clone()),
            Expr::And(expr1, expr2) => Expr::And(expr1.clone(), expr2.clone()),
            Expr::Or(expr1, expr2) => Expr::Or(expr1.clone(), expr2.clone())
        }
    }
}
