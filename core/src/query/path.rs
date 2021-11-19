use std::marker::PhantomData;
use std::ops::Add;

use generic_array::{arr, ArrayLength, GenericArray};
use generic_array::sequence::Concat;
use generic_array::typenum::{Sum, U1, UInt, UTerm};
use generic_array::typenum::bit::{B0, B1};

use interface::{FieldMarker, YukinoEntity};
use query_builder::Ident;

pub type True = B1;
pub type False = B0;

pub type FieldPath<F> = PathMarkerSeg<Terminal, F>;
pub type FieldPath2<F1, F2> = PathMarkerSeg<FieldPath<F2>, F1>;
pub type FieldPath3<F1, F2, F3> = PathMarkerSeg<FieldPath2<F2, F3>, F1>;
pub type FieldPath4<F1, F2, F3, F4> = PathMarkerSeg<FieldPath3<F2, F3, F4>, F1>;
pub type FieldPath5<F1, F2, F3, F4, F5> = PathMarkerSeg<FieldPath4<F2, F3, F4, F5>, F1>;
pub type List<P1> = PathMakerListSeg<Terminal, P1>;
pub type List2<P1, P2> = PathMakerListSeg<List<P1>, P2>;
pub type List3<P1, P2, P3> = PathMakerListSeg<List2<P1, P2>, P3>;
pub type List4<P1, P2, P3, P4> = PathMakerListSeg<List3<P1, P2, P3>, P4>;
pub type List5<P1, P2, P3, P4, P5> = PathMakerListSeg<List4<P1, P2, P3, P4>, P5>;
pub type ConcatList<L, R> = <L as ListConcat<<L as PathMarkerList>::Entity, R>>::Output;

pub type EntityOfField<F> = <F as FieldMarker>::Entity;
pub type EntityOfPath<P> = <P as PathMarker>::Entity;
pub type EntityOfList<L> = <L as PathMarkerList>::Entity;
pub type TypeOfField<F> = <F as FieldMarker>::FieldType;
pub type LengthOf<L> = <L as PathMarkerList>::L;

#[derive(Default)]
pub struct PathMarkerSeg<U, F: FieldMarker>(PhantomData<(U, F)>);

#[derive(Default)]
pub struct PathMakerListSeg<U: Sized, P: PathMarker + ?Sized>(PhantomData<(U, P)>);

#[derive(Default)]
pub struct Terminal;

pub trait PathMarker {
    type Entity: YukinoEntity;

    fn ident(pre: Ident) -> Ident
        where
            Self: Sized;
}

pub trait ListLength: ArrayLength<Ident> {}

pub trait PathMarkerList {
    type Entity: YukinoEntity;
    type L: ListLength + Add<U1>;

    fn idents(pre: Ident) -> GenericArray<Ident, Self::L>
        where
            Self: Sized;
}

pub trait SuitForGroupBy: PathMarker {}

pub trait SuitForGroupByList: PathMarkerList {}

pub trait InList<L: PathMarkerList>: PathMarker {}

pub trait ListConcat<E: YukinoEntity, R: PathMarkerList<Entity=E>>:
PathMarkerList<Entity=E>
{
    type Output: PathMarkerList<Entity=E>;
}

impl ListLength for UTerm {}

impl<N: ListLength> ListLength for UInt<N, B0> {}

impl<N: ListLength> ListLength for UInt<N, B1> {}

impl<F: FieldMarker> PathMarker for PathMarkerSeg<Terminal, F> {
    type Entity = EntityOfField<F>;

    fn ident(mut pre: Ident) -> Ident
        where
            Self: Sized,
    {
        pre.append_str(F::field_name());

        pre
    }
}

impl<U: PathMarker, F: FieldMarker<FieldType = EntityOfPath<U>>> PathMarker
    for PathMarkerSeg<U, F>
{
    type Entity = EntityOfField<F>;

    fn ident(mut pre: Ident) -> Ident
        where
            Self: Sized,
    {
        pre.append_str(F::field_name());

        pre
    }
}

impl<P: PathMarker> PathMarkerList for PathMakerListSeg<Terminal, P> {
    type Entity = EntityOfPath<P>;
    type L = U1;

    fn idents(pre: Ident) -> GenericArray<Ident, Self::L>
        where
            Self: Sized,
    {
        arr![Ident; P::ident(pre)]
    }
}

impl<E: YukinoEntity, U: PathMarkerList<Entity=E>, P: PathMarker<Entity=E>> PathMarkerList
for PathMakerListSeg<U, P>
    where
        Sum<LengthOf<U>, U1>: ListLength + Add<U1>,
{
    type Entity = E;
    type L = Sum<LengthOf<U>, U1>;

    fn idents(pre: Ident) -> GenericArray<Ident, Self::L>
        where
            Self: Sized,
    {
        Concat::concat(U::idents(pre.clone()), arr![Ident; P::ident(pre)])
    }
}

impl<F: FieldMarker<SuitForGroupBy=True>> SuitForGroupBy for PathMarkerSeg<Terminal, F> where
    Self: PathMarker
{}

impl<U: SuitForGroupBy, F: FieldMarker> SuitForGroupBy for PathMarkerSeg<U, F> where Self: PathMarker
{}

impl<P: SuitForGroupBy> SuitForGroupByList for PathMakerListSeg<Terminal, P> where
    Self: PathMarkerList
{}

impl<U: SuitForGroupByList, P: PathMarker> SuitForGroupByList for PathMakerListSeg<U, P> where
    Self: PathMarkerList
{}

impl<E: YukinoEntity> InList<PathMakerListSeg<Terminal, Self>> for dyn PathMarker<Entity=E> where
    PathMakerListSeg<Terminal, Self>: PathMarkerList
{}

impl<E: YukinoEntity, U: PathMarkerList<Entity=E>, P: PathMarker<Entity=E>>
InList<PathMakerListSeg<U, P>> for dyn PathMarker<Entity=E>
    where
        Self: InList<U>,
        PathMakerListSeg<U, Self>: PathMarkerList,
        Sum<LengthOf<U>, U1>: ListLength + Add<U1>,
{}

impl<L: PathMarkerList<Entity=E>, E: YukinoEntity, Pr: PathMarker<Entity=E>>
ListConcat<E, PathMakerListSeg<Terminal, Pr>> for L
    where
        Sum<LengthOf<L>, U1>: ListLength + Add<U1>,
{
    type Output = PathMakerListSeg<L, Pr>;
}

impl<
    L: PathMarkerList<Entity=E> + ListConcat<E, U>,
    E: YukinoEntity,
    U: PathMarkerList<Entity=E>,
    Pr: PathMarker<Entity=E>,
> ListConcat<E, PathMakerListSeg<U, Pr>> for L
    where
        Sum<LengthOf<U>, U1>: ListLength + Add<U1>,
        Sum<LengthOf<ConcatList<L, U>>, U1>: ListLength + Add<U1>,
{
    type Output = PathMakerListSeg<ConcatList<L, U>, Pr>;
}
