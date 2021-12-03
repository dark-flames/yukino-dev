use std::marker::PhantomData;

use crate::view::{EntityWithView, ExprView, Value};

pub type U0 = Zero;
pub type U1 = Suc<U0>;
pub type U2 = Suc<U1>;
pub type Len<L> = <L as BitMap>::L;
pub type EmptyTagList = BitMapSegment<BitMapSegment<BitMapSegment<Terminal, False>, False>, False>;
pub type AddTag<B, F> = <B as SetBit<<F as Tag>::Offset, True>>::Result;
pub type RemoveTag<B, F> = <B as SetBit<<F as Tag>::Offset, False>>::Result;
pub type TagList1<T1> = AddTag<EmptyTagList, T1>;
pub type TagList2<T1, T2> = AddTag<TagList1<T1>, T2>;
pub type TagList3<T1, T2, T3> = AddTag<TagList2<T1, T2>, T3>;
pub type TagOfValueView<T> = <<T as Value>::ValueExprView as ExprView<T>>::Tags;
pub type TagsOfEntity<E> = <<E as EntityWithView>::View as ExprView<E>>::Tags;
pub type OffsetOfTag<T> = <T as Tag>::Offset;
pub type ConcreteBitMap<L1, L2> = <L1 as MergeBitMap<Len<L1>, L2>>::Output;
pub type ConcreteList<L1, L2> = <L1 as MergeList<L2>>::Output;

#[derive(Default)]
pub struct Terminal;

#[derive(Default)]
pub struct Zero;

#[derive(Default)]
pub struct Suc<T>(PhantomData<T>);

#[derive(Default)]
pub struct True;
#[derive(Default)]
pub struct False;

#[derive(Default)]
pub struct BitMapSegment<U: BitMap, B: Bool>(PhantomData<(U, B)>);

pub trait TagList: BitMap<L = Len<EmptyTagList>> {}

pub trait BitMap: 'static {
    type L: Usize;
}

pub trait Usize: 'static {}

pub trait Bool: 'static {}

pub trait And<R: Bool> {
    type Result: Bool;
}

pub trait Or<R: Bool> {
    type Result: Bool;
}

pub trait AlwaysFalse<R: Bool> {
    type Result: Bool;
}

pub trait NonZero {
    type Prev: Usize;
}

pub trait Tag {
    type Offset: Usize;
}

pub trait AssertBit<O: Usize, V: Bool>: BitMap {}

pub trait SetBit<O: Usize, V: Bool>: BitMap {
    type Result: BitMap<L = Self::L>;
}

pub trait GetBit<O: Usize>: BitMap {
    type Result: Bool;
}

pub trait HasTag<T: Tag>: TagList {}

pub trait NoTag<T: Tag>: TagList {}

pub trait InList<L: TagList>: Tag {}

pub trait NotInList<L: TagList>: Tag {}

pub trait MergeList<R: TagList>: TagList {
    type Output: TagList;
}

pub trait MergeBitMap<L: Usize, R: BitMap<L = L>>: BitMap {
    type Output: BitMap<L = L>;
}

impl Usize for Zero {}

impl<P: Usize> Usize for Suc<P> {}

impl<P: Usize> NonZero for Suc<P> {
    type Prev = P;
}

impl Bool for True {}
impl Bool for False {}

impl BitMap for Terminal {
    type L = Zero;
}

impl<U: BitMap, B: Bool> BitMap for BitMapSegment<U, B> {
    type L = Suc<U::L>;
}

impl<U: BitMap> AssertBit<U0, True> for BitMapSegment<U, True> {}

impl<U: BitMap> AssertBit<U0, False> for BitMapSegment<U, False> {}

impl<U: BitMap + AssertBit<O::Prev, A>, O: NonZero + Usize, A: Bool, H: Bool> AssertBit<O, A>
    for BitMapSegment<U, H>
{
}

impl<U: BitMap, V: Bool, H: Bool> SetBit<U0, V> for BitMapSegment<U, H> {
    type Result = BitMapSegment<U, V>;
}

impl<U: BitMap, O: NonZero + Usize, V: Bool, H: Bool> SetBit<O, V> for BitMapSegment<U, H>
where
    U: SetBit<O::Prev, V>,
{
    type Result = BitMapSegment<<U as SetBit<O::Prev, V>>::Result, H>;
}

impl<U: BitMap, H: Bool> GetBit<U0> for BitMapSegment<U, H> {
    type Result = H;
}

impl<U: BitMap, O: NonZero + Usize, H: Bool> GetBit<O> for BitMapSegment<U, H>
where
    U: GetBit<O::Prev>,
{
    type Result = <U as GetBit<O::Prev>>::Result;
}

impl<T: Tag, B: TagList + AssertBit<T::Offset, True>> HasTag<T> for B {}

impl<T: Tag, B: TagList + AssertBit<T::Offset, False>> NoTag<T> for B {}

impl<T: Tag, L: TagList + HasTag<T> + AssertBit<T::Offset, True>> InList<L> for T {}
impl<T: Tag, L: TagList + NoTag<T> + AssertBit<T::Offset, False>> NotInList<L> for T {}

impl And<True> for True {
    type Result = True;
}

impl And<False> for True {
    type Result = False;
}

impl And<True> for False {
    type Result = False;
}

impl And<False> for False {
    type Result = False;
}

impl Or<True> for True {
    type Result = True;
}

impl Or<False> for True {
    type Result = True;
}

impl Or<True> for False {
    type Result = True;
}

impl Or<False> for False {
    type Result = False;
}

impl<L: Bool, R: Bool> AlwaysFalse<R> for L {
    type Result = False;
}

impl MergeBitMap<Zero, Terminal> for Terminal {
    type Output = Terminal;
}

impl<L: TagList + MergeBitMap<Len<EmptyTagList>, R>, R: TagList> MergeList<R> for L {
    type Output = <L as MergeBitMap<Len<EmptyTagList>, R>>::Output;
}

impl<B: BitMap<L = Len<EmptyTagList>>> TagList for B {}

macro_rules! create_tag {
    ($name: ident, $offset: ty, $strategy: ident) => {
        pub struct $name;

        impl Tag for $name {
            type Offset = $offset;
        }

        impl<
                L: BitMap<L = $offset> + MergeBitMap<$offset, R>,
                R: BitMap<L = $offset>,
                LH: Bool + $strategy<RH>,
                RH: Bool,
            > MergeBitMap<Suc<$offset>, BitMapSegment<R, RH>> for BitMapSegment<L, LH>
        {
            type Output = BitMapSegment<
                <L as MergeBitMap<$offset, R>>::Output,
                <LH as $strategy<RH>>::Result,
            >;
        }
    };
}

create_tag!(OrdViewTag, U0, And);
create_tag!(EntityViewTag, U1, AlwaysFalse);
create_tag!(AggregateViewTag, U2, And);

#[cfg(test)]
mod test {
    use crate::view::{
        AggregateViewTag, AssertBit, ConcreteList, EmptyTagList, EntityViewTag, False, InList,
        NotInList, OffsetOfTag, OrdViewTag, Tag, TagList, TagList1, TagList2, TagList3,
    };

    type A = TagList3<OrdViewTag, EntityViewTag, AggregateViewTag>;
    type B = TagList2<OrdViewTag, EntityViewTag>;
    type R = TagList1<OrdViewTag>;
    type C = ConcreteList<A, B>;

    fn assert_in_list<T: Tag + InList<List>, List: TagList>() {}
    fn assert_not_in_list<
        T: Tag + NotInList<List>,
        List: TagList + AssertBit<OffsetOfTag<T>, False>,
    >() {
    }

    #[test]
    fn test() {
        let _a: R = C::default();
        assert_in_list::<EntityViewTag, A>();
        assert_not_in_list::<EntityViewTag, R>();
        assert_not_in_list::<EntityViewTag, EmptyTagList>();
    }
}
