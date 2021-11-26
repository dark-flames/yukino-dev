use std::marker::PhantomData;

use crate::view::{ExprView, Value};

pub type U0 = Zero;
pub type U1 = Suc<U0>;
pub type EmptyTagList = TagListSegment<TagListSegment<Terminal, false>, false>;
pub type SetFlag<B, F> = <B as SetBit<true, <F as Tag>::Offset>>::Result;
pub type TagList1<T1> = SetFlag<EmptyTagList, T1>;
pub type TagList2<T1, T2> = SetFlag<TagList1<T1>, T2>;
pub type TagList3<T1, T2, T3> = SetFlag<TagList2<T1, T2>, T3>;
pub type TagOfValueView<T> = <<T as Value>::ValueExprView as ExprView<T>>::Tags;

pub struct Terminal;

pub struct Zero;

pub struct Suc<T>(PhantomData<T>);

pub struct TagListSegment<U: TagList, const B: bool>(PhantomData<U>);

pub trait TagList: 'static {}

pub trait Usize {}

pub trait NonZero {
    type Prev: Usize;
}

pub trait Tag {
    type Offset: Usize;
}

pub trait AssertBit<const V: bool, O: Usize>: TagList {}

pub trait SetBit<const V: bool, O: Usize>: TagList {
    type Result: TagList;
}

pub trait HasTag<T: Tag>: TagList {}

pub trait InList<L: TagList>: Tag {}

impl Usize for Zero {}

impl<P: Usize> Usize for Suc<P> {}

impl<P: Usize> NonZero for Suc<P> {
    type Prev = P;
}

impl TagList for Terminal {}

impl<U: TagList, const B: bool> TagList for TagListSegment<U, B> {}

impl<U: TagList, const B: bool> AssertBit<B, U0> for TagListSegment<U, B> {}

impl<U: TagList, O: NonZero + Usize, const V: bool, const H: bool> AssertBit<V, O>
    for TagListSegment<U, H>
where
    U: AssertBit<V, O::Prev>,
{
}

impl<U: TagList, const V: bool, const H: bool> SetBit<V, U0> for TagListSegment<U, H> {
    type Result = TagListSegment<U, V>;
}

impl<U: TagList, O: NonZero + Usize, const V: bool, const H: bool> SetBit<V, O>
    for TagListSegment<U, H>
where
    U: SetBit<V, O::Prev>,
{
    type Result = TagListSegment<<U as SetBit<V, O::Prev>>::Result, H>;
}

impl<T: Tag, B: TagList> HasTag<T> for B where B: AssertBit<true, T::Offset> {}

impl<T: Tag, L: TagList + HasTag<T>> InList<L> for T {}

macro_rules! create_tag {
    ($name: ident, $offset: ty) => {
        pub struct $name;

        impl Tag for $name {
            type Offset = $offset;
        }
    };
}

create_tag!(EntityViewTag, U0);
create_tag!(AggregateViewTag, U1);
