use generic_array::arr;

use query_builder::Expr;

use crate::view::{
    ConcreteList, ExprView, ExprViewBoxWithTag, MergeList, SingleExprView, TagList, TagOfValueView,
    Value,
};

macro_rules! op_trait {
    (
        $op_trait: ident,
        $op_trait_method: ident
    ) => {
        pub trait $op_trait<Rhs = Self> {
            fn $op_trait_method(&self, rhs: &Rhs) -> bool;
        }
    };
}
macro_rules! impl_op_for {
    (
        $op: tt,
        $op_trait: ident,
        $op_trait_method: ident,
        [$($ty: ty),*]
    ) => {
        $(
        impl $op_trait for $ty {
            fn $op_trait_method(&self, rhs: &Self) -> bool {
                self $op rhs
            }
        }
        )*
    }
}
macro_rules! impl_expr_for {
    (
        $expr_op_trait: ident,
        $expr_op_method: ident,
        $expr_variant: ident,
        [$($ty: ty),*]
    ) => {
        $(
        impl $expr_op_trait for $ty {
            type ResultTags<LTags: TagList + MergeList<RTags>, RTags: TagList> = ConcreteList<LTags, RTags>;
            fn $expr_op_method<LTags: TagList + MergeList<RTags>, RTags: TagList>(
                l: ExprViewBoxWithTag<Self, LTags>,
                r: ExprViewBoxWithTag<$ty, RTags>
            ) -> ExprViewBoxWithTag<bool, Self::ResultTags<LTags, RTags>> {
                let l_expr = l.collect_expr().into_iter().next().unwrap();
                let r_expr = r.collect_expr().into_iter().next().unwrap();
                let result = Expr::$expr_variant(Box::new(l_expr), Box::new(r_expr));
                SingleExprView::from_exprs(arr![Expr; result])
            }
        }
        )*
    }
}
macro_rules! impl_bool_operator {
    (
        $op: tt,
        $op_trait: ident,
        $op_trait_method: ident,
        $view_op_trait: ident,
        $view_op_method: ident,
        $expr_op_trait: ident,
        $expr_op_method: ident,
        $expr_variant: ident,
        [$($ty: ty),*]
    ) => {
        pub trait $view_op_trait<Rhs = Self> {
            type Output;
            fn $view_op_method(self, rhs: Rhs) -> Self::Output;
        }


        pub trait $expr_op_trait<Rhs: Value = Self>: Value + $op_trait<Rhs> {
            type ResultTags<LTags: TagList + MergeList<RTags>, RTags: TagList>: TagList;

            fn $expr_op_method<LTags: TagList + MergeList<RTags>, RTags: TagList>(
                l: ExprViewBoxWithTag<Self, LTags>,
                r: ExprViewBoxWithTag<Rhs, RTags>
            ) -> ExprViewBoxWithTag<bool, Self::ResultTags<LTags, RTags>>;
        }

        impl<
            L: Value + $expr_op_trait<R>,
            R: Value,
            LTags: TagList + MergeList<RTags>,
            RTags: TagList
        > $view_op_trait<ExprViewBoxWithTag<R, RTags>>
            for ExprViewBoxWithTag<L, LTags>
        {
            type Output = ExprViewBoxWithTag<bool, <L as $expr_op_trait<R>>::ResultTags<LTags, RTags>>;

            fn $view_op_method(self, rhs: ExprViewBoxWithTag<R, RTags>) -> Self::Output {
                L::$expr_op_method(self, rhs)
            }
        }
        impl<
            L: Value + $expr_op_trait<R>,
            R: Value,
            LTags: TagList + MergeList<TagOfValueView<R>>,
        > $view_op_trait<R> for ExprViewBoxWithTag<L, LTags> {
            type Output = ExprViewBoxWithTag<bool, <L as $expr_op_trait<R>>::ResultTags<LTags, TagOfValueView<R>>>;

            fn $view_op_method(self, rhs: R) -> Self::Output {
                L::$expr_op_method(self, rhs.view())
            }
        }

        impl_expr_for! (
            $expr_op_trait,
            $expr_op_method,
            $expr_variant,
            [$($ty),*]
        );
    };
}

macro_rules! generate_macro {
    ($name: ident, $view_trait: ident, $view_trait_method: ident) => {
        #[macro_export]
        macro_rules! $name {
            ($l: expr, $r: expr) => {{
                use yukino::operator::$view_trait;
                ($l).$view_trait_method($r)
            }};
        }
    };
}

op_trait!(And, and);
op_trait!(Or, or);
op_trait!(Bt, bt);
op_trait!(Bte, bte);
op_trait!(Lt, lt);
op_trait!(Lte, lte);

impl_op_for!(&, And, and, [bool]);
impl_op_for!(|, Or, or, [bool]);
impl_op_for!(>, Bt, bt, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(>=, Bte, bte, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(<, Lt, lt, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_op_for!(<=, Lte, lte, [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);

impl_bool_operator!(&, And, and, ViewAnd, view_and, ExprAnd, expr_and, And, [bool]);
impl_bool_operator!(|, Or, or, ViewOr, view_or, ExprOr, expr_or, Or, [bool]);
impl_bool_operator!(==, PartialEq, eq, ViewEq, view_eq, ExprEq, expr_eq, Eq,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(!=, PartialEq, ne, ViewNeq, view_neq, ExprNeq, expr_neq, Neq,
    [bool, u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(>, Bt, bt, ViewBt, view_bt, ExprBt, expr_bt, Bt,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(>=, Bte, bte, ViewBte, view_bte, ExprBte, expr_bte, Bte,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(<, Lt, lt, ViewLt, view_lt, ExprLt, expr_lt, Lt,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_bool_operator!(<=, Lte, lte, ViewLte, view_lte, ExprLte, expr_lte, Lte,
    [u16, i16, u32, i32, u64, i64, f32, f64, char, String]);

generate_macro!(and, ViewAnd, view_and);
generate_macro!(or, ViewOr, view_or);
generate_macro!(eq, ViewEq, view_eq);
generate_macro!(neq, ViewNeq, view_neq);
generate_macro!(bt, ViewBt, view_bt);
generate_macro!(bte, ViewBte, view_bte);
generate_macro!(lt, ViewLt, view_lt);
generate_macro!(lte, ViewLte, view_lte);
