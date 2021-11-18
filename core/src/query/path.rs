use generic_array::typenum::bit::{B0, B1};
use interface::{FieldMarker, YukinoEntity};
use query_builder::Ident;
use std::marker::PhantomData;

pub type True = B1;
pub type False = B0;

pub type FieldPath<F> = PathMarkerSeg<PathMarkerTerm, F>;
pub type FieldPath2<F1, F2> = PathMarkerSeg<FieldPath<F2>, F1>;
pub type FieldPath3<F1, F2, F3> = PathMarkerSeg<FieldPath2<F2, F3>, F1>;
pub type FieldPath4<F1, F2, F3, F4> = PathMarkerSeg<FieldPath3<F2, F3, F4>, F1>;
pub type FieldPath5<F1, F2, F3, F4, F5> = PathMarkerSeg<FieldPath4<F2, F3, F4, F5>, F1>;

pub type EntityOfField<F> = <F as FieldMarker>::Entity;
pub type EntityOfPath<P> = <P as PathMarker>::Entity;
pub type TypeOfField<F> = <F as FieldMarker>::FieldType;

pub struct PathMarkerSeg<U, F: FieldMarker>(PhantomData<(U, F)>);

pub struct PathMarkerTerm;

pub trait PathMarker {
    type Entity: YukinoEntity;

    fn ident(pre: Ident) -> Ident
    where
        Self: Sized;
}

pub trait SuitForGroupBy: PathMarker {}

impl<F: FieldMarker> PathMarker for PathMarkerSeg<PathMarkerTerm, F> {
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

impl<F: FieldMarker<SuitForGroupBy = True>> SuitForGroupBy for PathMarkerSeg<PathMarkerTerm, F> where
    Self: PathMarker
{
}
impl<U: SuitForGroupBy, F: FieldMarker> SuitForGroupBy for PathMarkerSeg<U, F> where Self: PathMarker
{}
