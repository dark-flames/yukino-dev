use std::marker::PhantomData;

use crate::view::{EntityWithView, ExprView, Value};

pub type U0 = Zero;
pub type U1 = Suc<U0>;
pub type U2 = Suc<U1>;
pub type EmptyTagList =
    TagListSegment<TagListSegment<TagListSegment<Terminal, false>, false>, false>;
pub type AddTag<B, F> = <B as SetBit<<F as Tag>::Offset, true>>::Result;
pub type RemoveTag<B, F> = <B as SetBit<<F as Tag>::Offset, false>>::Result;
pub type TagList1<T1> = AddTag<EmptyTagList, T1>;
pub type TagList2<T1, T2> = AddTag<TagList1<T1>, T2>;
pub type TagList3<T1, T2, T3> = AddTag<TagList2<T1, T2>, T3>;
pub type TagOfValueView<T> = <<T as Value>::ValueExprView as ExprView<T>>::Tags;
pub type TagsOfEntity<E> = <<E as EntityWithView>::View as ExprView<E>>::Tags;

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

pub trait AssertBit<O: Usize, const V: bool>: TagList {}

pub trait SetBit<O: Usize, const V: bool>: TagList {
    type Result: TagList;
}

pub trait HasTag<T: Tag>: TagList {}

pub trait NoTag<T: Tag>: TagList {}

pub trait InList<L: TagList>: Tag {}

pub trait NotInList<L: TagList>: Tag {}

impl Usize for Zero {}

impl<P: Usize> Usize for Suc<P> {}

impl<P: Usize> NonZero for Suc<P> {
    type Prev = P;
}

impl TagList for Terminal {}

impl<U: TagList, const B: bool> TagList for TagListSegment<U, B> {}

impl<U: TagList, const B: bool> AssertBit<U0, B> for TagListSegment<U, B> {}

impl<U: TagList, O: NonZero + Usize, const V: bool, const H: bool> AssertBit<O, V>
    for TagListSegment<U, H>
where
    U: AssertBit<O::Prev, V>,
{
}

impl<U: TagList, const V: bool, const H: bool> SetBit<U0, V> for TagListSegment<U, H> {
    type Result = TagListSegment<U, V>;
}

impl<U: TagList, O: NonZero + Usize, const V: bool, const H: bool> SetBit<O, V>
    for TagListSegment<U, H>
where
    U: SetBit<O::Prev, V>,
{
    type Result = TagListSegment<<U as SetBit<O::Prev, V>>::Result, H>;
}

impl<T: Tag, B: TagList> HasTag<T> for B where B: AssertBit<T::Offset, true> {}

impl<T: Tag, B: TagList> NoTag<T> for B where B: AssertBit<T::Offset, false> {}

impl<T: Tag, L: TagList + HasTag<T>> InList<L> for T {}
impl<T: Tag, L: TagList + NoTag<T>> NotInList<L> for T {}

macro_rules! create_tag {
    ($name: ident, $offset: ty) => {
        pub struct $name;

        impl Tag for $name {
            type Offset = $offset;
        }
    };
}

create_tag!(OrdViewTag, U0);
create_tag!(EntityViewTag, U1);
create_tag!(AggregateViewTag, U2);
