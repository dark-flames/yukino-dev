use std::marker::PhantomData;

use crate::view::{ExprView, Value};

pub type EmptyTagList = Terminal;
pub type TagList1<T1> = TagListSegment<Terminal, T1>;
pub type TagList2<T1, T2> = TagListSegment<TagList1<T1>, T2>;
pub type TagList3<T1, T2, T3> = TagListSegment<TagList2<T1, T2>, T3>;
pub type TagOfValueView<T> = <<T as Value>::ValueExprView as ExprView<T>>::Tags;

pub trait Tag: 'static {}

pub trait TagList: 'static {}

pub trait InList<L: TagList>: Tag {}

pub struct Terminal;

pub struct TagListSegment<U: TagList + Sized, T: Tag + ?Sized>(PhantomData<(U, T)>);

impl TagList for Terminal {}

impl<U: TagList, T: Tag> TagList for TagListSegment<U, T> {}

impl<U: TagList> InList<TagListSegment<U, Self>> for dyn Tag where TagListSegment<U, Self>: TagList {}

impl<U: TagList, T: Tag> InList<TagListSegment<U, T>> for dyn Tag
where
    Self: InList<U>,
    TagListSegment<U, Self>: TagList,
{
}

macro_rules! create_tag {
    ($name: ident) => {
        pub struct $name;

        impl Tag for $name {}
    };
}

create_tag!(EntityViewTag);
create_tag!(AggregateViewTag);
