use crate::db::ty::{DatabaseType, DatabaseValue};
use crate::err::{ErrorOnExpr, ExprResult, TypeError};
use crate::query::{FunctionCall, Ident};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type ExprBox = Box<Expr>;

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

pub struct TypedExpr {
    expr: ExprBox,
    ty: DatabaseType,
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

impl TypedExpr {
    pub fn ident(ident: Ident) -> ExprResult<Self> {
        let ty = ident.ty;
        Ok(TypedExpr {
            expr: Box::new(Expr::Ident(ident)),
            ty,
        })
    }

    pub fn lit(lit: DatabaseValue) -> ExprResult<Self> {
        let ty = DatabaseType::from(&lit);
        Ok(TypedExpr {
            expr: Box::new(Expr::Lit(lit)),
            ty,
        })
    }

    pub fn fn_call(f: FunctionCall) -> ExprResult<Self> {
        let ty = f.return_ty;
        Ok(TypedExpr {
            expr: Box::new(Expr::FunctionCall(f)),
            ty,
        })
    }

    pub fn bit_inverse(self) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("~".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::BitInverse(self.expr)),
                ty,
            })
        }
    }

    pub fn bit_xor(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;
        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("^".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("^".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::BitXor(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn multi(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.mul_operate() {
            Err(TypeError::UnimplementedOperator("*".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("*".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Mul(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn divide(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.mul_operate() {
            Err(TypeError::UnimplementedOperator("/".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("/".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Div(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn plus(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.add_operate() {
            Err(TypeError::UnimplementedOperator("+".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("+".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Add(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn minus(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.add_operate() {
            Err(TypeError::UnimplementedOperator("-".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("-".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Sub(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn left_shift(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("<<".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator(">>".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::LeftShift(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn right_shift(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator(">>".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator(">>".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::RightShift(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn bit_and(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("&".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("&".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::BitAnd(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn bit_or(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("|".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("|".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::BitOr(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn bte(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.ord() {
            Err(TypeError::UnimplementedOperator(">=".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator(">=".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Bte(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn lte(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.ord() {
            Err(TypeError::UnimplementedOperator("<=".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("<=".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Bte(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn neq(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.eq() {
            Err(TypeError::UnimplementedOperator("!=".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("!=".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Neq(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn bt(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.ord() {
            Err(TypeError::UnimplementedOperator(">".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator(">".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Bt(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn lt(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.ord() {
            Err(TypeError::UnimplementedOperator("<".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("<".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Bt(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn eq(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.eq() {
            Err(TypeError::UnimplementedOperator("=".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("=".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Eq(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn inverse(self) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.bit_operate() {
            Err(TypeError::UnimplementedOperator("!".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Not(self.expr)),
                ty,
            })
        }
    }

    pub fn and(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.logic_operate() {
            Err(TypeError::UnimplementedOperator("AND".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("AND".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::And(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn xor(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.logic_operate() {
            Err(TypeError::UnimplementedOperator("XOR".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("XOR".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Xor(self.expr, r.expr)),
                ty,
            })
        }
    }

    pub fn or(self, r: TypedExpr) -> ExprResult<Self> {
        let ty = self.ty;

        if !ty.logic_operate() {
            Err(TypeError::UnimplementedOperator("OR".to_string(), ty)
                .as_expr_err(self.expr.as_ref()))
        } else if ty != r.ty {
            Err(TypeError::CannotApplyOperator("OR".to_string(), ty, r.ty)
                .as_expr_err(self.expr.as_ref()))
        } else {
            Ok(TypedExpr {
                expr: Box::new(Expr::Or(self.expr, r.expr)),
                ty,
            })
        }
    }
}
