use std::marker::PhantomData;

use generic_array::{arr, GenericArray};
use generic_array::typenum::U1;

use query_builder::{DatabaseValue, Expr, ExprMutVisitor, ExprNode, ExprVisitor, Function, FunctionCall};

use crate::err::{RuntimeResult, YukinoError};
use crate::view::{ExprView, ExprViewBox, Value, ValueCountOf, View, ViewBox};

#[derive(Clone)]
pub struct AggregateView<T: Value<L=U1>> {
    function_call: FunctionCall,
    _marker: PhantomData<T>,
}

impl<T: Value<L=U1>> ExprNode for AggregateView<T> {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.function_call.apply(visitor);
    }

    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.function_call.apply_mut(visitor);
    }
}

impl<T: Value<L=U1>> View<T, ValueCountOf<T>> for AggregateView<T> {
    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<T>> {
        arr![Expr; Expr::FunctionCall(self.function_call.clone())]
    }

    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<T>>) -> RuntimeResult<T> {
        (*T::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }

    fn view_clone(&self) -> ViewBox<T, ValueCountOf<T>> {
        Box::new(self.clone())
    }
}

impl<T: Value<L=U1>> ExprView<T> for AggregateView<T> {
    fn from_exprs(_exprs: GenericArray<Expr, ValueCountOf<T>>) -> Self
        where
            Self: Sized,
    {
        unreachable!("AggregateView cannot be construct directly");
    }

    fn expr_clone(&self) -> ExprViewBox<T> {
        Box::new(self.clone())
    }
}

impl<T: Value<L=U1>> AggregateView<T> {
    pub fn from_function_call(f: FunctionCall) -> Self {
        AggregateView {
            function_call: f,
            _marker: Default::default(),
        }
    }
}

macro_rules! impl_aggregate {
    (
        $trait_name: ident,
        $trait_method: ident,
        $method_name: ident,
        $macro_name: ident,
        $variant: ident,
        [$($types: ty),*]
    ) => {
        pub trait $trait_name: Value {
            type Result: Value<L=U1>;

            fn $trait_method(expr: ExprViewBox<Self>) -> AggregateView<Self::Result>;
        }

        pub fn $method_name<V: Value + $trait_name>(
            v: ExprViewBox<V>,
        ) -> AggregateView<<V as $trait_name>::Result> {
            V::$trait_method(v)
        }

        macro_rules! $macro_name {
            ($ty: ty) => {
                impl $trait_name for $ty {
                    type Result = $ty;

                    fn $trait_method(expr: ExprViewBox<Self>) -> AggregateView<Self::Result> {
                        AggregateView::from_function_call(FunctionCall {
                            func: Function::$variant,
                            params: vec![expr.collect_expr().into_iter().next().unwrap()]
                        })
                    }
                }
            };
        }

        $($macro_name!($types);)*
    }
}

impl_aggregate!(ExprsAverage, exprs_average, average, impl_average, Average, [
    u16, i16, u32, i32, u64, i64, f32, f64]);
impl_aggregate!(ExprsBitAnd, exprs_bit_and, bit_and, impl_bit_and, BitAnd, [
    u16, i16, u32, i32, u64, i64]);
impl_aggregate!(ExprsBitOr, exprs_bit_or, bit_or, impl_bit_or, BitOr, [
    u16, i16, u32, i32, u64, i64]);
impl_aggregate!(ExprsBitXor, exprs_bit_xor, bit_xor, impl_bit_xor, BitXor, [
    u16, i16, u32, i32, u64, i64]);
impl_aggregate!(ExprsConcat, exprs_concat, concat, impl_concat, Concat, [String]);
impl_aggregate!(ExprsMin, exprs_min, min, impl_min, Min, [
    u16, i16, u32, i32, u64, i64, f32, f64, char, String]);
impl_aggregate!(ExprsMax, exprs_max, max, impl_max, Max, [
    u16, i16, u32, i32, u64, i64, f32, f64, char, String]);

impl ExprsConcat for char {
    type Result = String;

    fn exprs_concat(expr: ExprViewBox<Self>) -> AggregateView<Self::Result> {
        AggregateView::from_function_call(FunctionCall {
            func: Function::Concat,
            params: vec![expr.collect_expr().into_iter().next().unwrap()],
        })
    }
}
