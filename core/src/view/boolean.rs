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
