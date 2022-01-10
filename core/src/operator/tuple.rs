use std::ops::{Add, Sub};

use crate::view::{
    AnyTagsValue, ConcreteList, ExprViewBoxWithTag, MergeList, TagList, TagsOfValueView,
    TupleExprView, Value, ValueCountOf,
};

impl<L: Value, R: Value, LTags: TagList + MergeList<RTags>, RTags: TagList>
    From<(ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), ConcreteList<LTags, RTags>>
where
    (L, R): Value,
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r))
    }
}

impl<L: Value, R: AnyTagsValue, LTags: TagList + MergeList<LTags>>
    From<(ExprViewBoxWithTag<L, LTags>, R)>
    for ExprViewBoxWithTag<(L, R), ConcreteList<LTags, LTags>>
where
    (L, R): Value,
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (ExprViewBoxWithTag<L, LTags>, R)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l, r.view_with_tags::<LTags>()))
    }
}

impl<L: AnyTagsValue, R: Value, RTags: TagList> From<(L, ExprViewBoxWithTag<R, RTags>)>
    for ExprViewBoxWithTag<(L, R), ConcreteList<RTags, RTags>>
where
    (L, R): Value,
    RTags: MergeList<RTags>,
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>, Output = ValueCountOf<(L, R)>>,
    ValueCountOf<(L, R)>: Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    fn from(tuple: (L, ExprViewBoxWithTag<R, RTags>)) -> Self {
        let (l, r) = tuple;
        Box::new(TupleExprView(l.view_with_tags::<RTags>(), r))
    }
}
