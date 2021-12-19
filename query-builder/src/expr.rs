use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use crate::{DatabaseValue, FunctionCall, Ident, QueryBuildState, SelectQuery, ToSql};

pub type ExprBox = Box<Expr>;

#[derive(Clone, Debug)]
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
    In(ExprBox, SelectQuery),
    Exists(SelectQuery),
    NotExists(SelectQuery),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expr::Ident(i) => i.fmt(f),
            Expr::Lit(l) => l.fmt(f),
            Expr::FunctionCall(c) => Display::fmt(&c, f),
            Expr::Subquery(s) => write!(f, "({})", s),
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
            Expr::In(l, r) => write!(f, "{} IN ({})", l, r),
            Expr::Exists(s) => write!(f, "EXISTS ({})", s),
            Expr::NotExists(s) => write!(f, "NOT EXISTS ({})", s),
        }
    }
}

impl ToSql for Expr {
    fn to_sql(&self, state: &mut QueryBuildState) -> FmtResult {
        match self {
            Expr::Ident(ident) => ident.to_sql(state),
            Expr::Lit(l) => l.to_sql(state),
            Expr::FunctionCall(f) => f.to_sql(state),
            Expr::Subquery(query) => {
                write!(state, "(")?;
                query.to_sql(state)?;
                write!(state, ")")
            }
            Expr::BitInverse(e) => {
                write!(state, "(")?;
                write!(state, "~")?;
                e.to_sql(state)?;
                write!(state, ")")
            }
            Expr::BitXor(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "^")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Mul(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "*")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Div(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "/")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Rem(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "%")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Add(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "+")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Sub(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "-")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::LeftShift(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "<<")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::RightShift(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, ">>")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::BitAnd(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "&")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::BitOr(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "|")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Bte(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, ">=")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Lte(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "<=")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Neq(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "!=")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Bt(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, ">")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Lt(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "<")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Eq(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "=")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Not(e) => {
                write!(state, "(")?;
                write!(state, "Not")?;
                e.to_sql(state)?;
                write!(state, ")")
            }
            Expr::And(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "AND")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::Or(l, r) => {
                write!(state, "(")?;
                l.to_sql(state)?;
                write!(state, "OR")?;
                r.to_sql(state)?;
                write!(state, ")")
            }
            Expr::In(e, s) => {
                write!(state, "(")?;
                e.to_sql(state)?;
                write!(state, "IN")?;
                write!(state, "(")?;
                s.to_sql(state)?;
                write!(state, ")")?;
                write!(state, ")")
            }
            Expr::Exists(s) => {
                write!(state, "(")?;
                write!(state, "EXISTS")?;
                write!(state, "(")?;
                s.to_sql(state)?;
                write!(state, ")")?;
                write!(state, ")")
            }
            Expr::NotExists(s) => {
                write!(state, "(")?;
                write!(state, "NOT EXISTS")?;
                write!(state, "(")?;
                s.to_sql(state)?;
                write!(state, ")")?;
                write!(state, ")")
            }
        }
    }
}

unsafe impl Send for Expr {}
unsafe impl Sync for Expr {}
