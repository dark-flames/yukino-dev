use std::ops::{Add, Sub};
use generic_array::GenericArray;
use generic_array::sequence::Split;
use crate::db::ty::DatabaseValue;
use crate::err::RuntimeResult;
use crate::view::{ComputationView, ComputationViewBox, ExprViewBox, Value, ValueCount, View, ViewBox};
macro_rules! bool_operator_trait {
    ($name: ident, $method: ident) => {
        pub trait $name<Rhs=Self> {
            fn $method(self, rhs: Rhs) -> bool;
        }
    }
}

macro_rules! impl_for_bool {
    ($name: ident, $method: ident, $op: tt) => {
        impl $name for bool {
            fn $method(self, rhs: Self) -> bool {
                self $op rhs
            }
        }
    };
}

macro_rules! impl_for_eq {
    ($name: ident, $method: ident, $op: tt) => {
        impl<T: PartialEq> $name for T {
            fn $method(self, rhs: Self) -> bool {
                self $op rhs
            }
        }
    };
}

macro_rules! impl_for_ord {
    ($name: ident, $method: ident, $op: tt) => {
        impl<T: Ord> $name for T {
            fn $method(self, rhs: Self) -> bool {
                self $op rhs
            }
        }
    };
}

bool_operator_trait!(And, and);
bool_operator_trait!(Or, or);
bool_operator_trait!(Eq, eq);
bool_operator_trait!(Neq, neq);
bool_operator_trait!(Bt, bt);
bool_operator_trait!(Bte, bte);
bool_operator_trait!(Lt, lt);
bool_operator_trait!(Lte, lte);
impl_for_bool!(And, and, &&);
impl_for_bool!(Or, or, ||);
impl_for_eq!(Eq, eq, ==);
impl_for_eq!(Neq, neq, !=);
impl_for_ord!(Bt, bt, >);
impl_for_ord!(Bte, bte, >=);
impl_for_ord!(Lt, lt, <);
impl_for_ord!(Lte, lte, <=);

pub trait ExprAnd<Rhs: Value>: Value {
    fn and(
        l: ExprViewBox<Self, <Self as Value>::L>,
        r: ExprViewBox<Rhs, <Rhs as Value>::L>
    ) -> ExprViewBox<bool, <bool as Value>::L>;
}

pub struct AndView<
    L: 'static + And<R>,
    R: 'static,
    LL: ValueCount,
    RL: ValueCount
> {
    l: ViewBox<L, LL>,
    r: ViewBox<R, RL>,
}

impl<
    L: 'static + And<R>,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> View<bool, OL> for AndView<L, R, LL, RL> {
    fn eval(&self, v: &GenericArray<DatabaseValue, OL>) -> RuntimeResult<bool> {
        let (l, r) = Split::<_, LL>::split(v);
        Ok(self.l.eval(l)?.and(self.r.eval(r)?))
    }

    fn view_clone(&self) -> ViewBox<bool, OL> {
        Box::new(AndView {
            l: self.l.view_clone(),
            r: self.r.view_clone(),
        })
    }
}

impl <
    L: 'static + And<R>,
    R: 'static,
    LL: ValueCount + Add<RL, Output=OL>,
    RL: ValueCount,
    OL: ValueCount + Sub<LL, Output=RL>,
> ComputationView<bool, OL> for AndView<L, R, LL, RL> {
    fn computation_clone(&self) -> ComputationViewBox<bool, OL> {
        Box::new(AndView {
            l: self.l.view_clone(),
            r: self.r.view_clone(),
        })
    }
}