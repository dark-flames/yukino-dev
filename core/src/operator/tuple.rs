use std::ops::{Add, Sub};

use crate::view::{EmptyTagList, ExprViewBoxWithTag, TagList, TupleExprView, Value, ValueCountOf};

impl<L: Value, R: Value, LTags: TagList, RTags: TagList>
    From<(ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r))
    }
}

impl<L: Value, R: Value, LTags: TagList> From<(ExprViewBoxWithTag<L, LTags>, R)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, R)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r.view()))
    }
}

impl<L: Value, R: Value, RTags: TagList> From<(L, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), EmptyTagList>
where
    (L, R): Value,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (L, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l.view(), r))
    }
}

#[macro_export]
macro_rules! tuple {
    ($l: expr, $r: expr) => {
        ($l, $r).into()
    };
}
