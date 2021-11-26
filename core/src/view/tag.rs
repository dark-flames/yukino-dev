use std::marker::PhantomData;

use crate::view::{ExprView, Value};

pub type EmptyTagList = Terminal;
pub type TagList1<T1> = TagListSegment<Terminal, T1>;
pub type TagList2<T1, T2> = TagListSegment<TagList1<T1>, T2>;
pub type TagList3<T1, T2, T3> = TagListSegment<TagList2<T1, T2>, T3>;
pub type TagOfValueView<T> = <<T as Value>::ValueExprView as ExprView<T>>::Tags;

pub trait Tag: 'static {}

pub trait TagList: 'static {}

pub trait Different<T: Tag>: Tag {}

pub trait InList<L: TagList>: Tag {}

pub struct Terminal;

pub struct TagListSegment<U: TagList + Sized, T: Tag + ?Sized>(PhantomData<(U, T)>);

impl TagList for Terminal {}

impl<U: TagList, T: Tag> TagList for TagListSegment<U, T> {}

impl<U: TagList, T: Tag> InList<TagListSegment<U, T>> for T where TagListSegment<U, T>: TagList {}

// todo: in list

macro_rules! create_tag {
    ($name: ident) => {
        pub struct $name;

        impl Tag for $name {}
    };
}

create_tag!(EntityViewTag);
create_tag!(AggregateViewTag);

// workaround for type inequality

impl Different<EntityViewTag> for AggregateViewTag {}
impl Different<AggregateViewTag> for EntityViewTag {}

#[cfg(test)]
fn bound<L: TagList>() where AggregateViewTag: InList<L> {}

#[test]
fn test_in_list() {
    bound::<TagList1<AggregateViewTag>>();
}
