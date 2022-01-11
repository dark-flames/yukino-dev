use std::marker::PhantomData;
use std::ops::{Add, Sub};

use generic_array::{
    GenericArray,
    sequence::{Concat, Split},
    typenum::Sum,
};

use query_builder::DatabaseValue;

use crate::converter::{Converter, ConverterInstance, ConvertResult};
use crate::view::{MergeList, TagsOfValueView, Value, ValueCount, ValueCountOf};

pub struct TupleConverter<L: Value, R: Value>(PhantomData<(L, R)>);

impl<L: Value, R: Value> Converter for TupleConverter<L, R>
where
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    type Output = (L, R);

    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &Self::INSTANCE
    }

    fn deserialize(
        &self,
        data: GenericArray<DatabaseValue, ValueCountOf<Self::Output>>,
    ) -> ConvertResult<Self::Output> {
        let (v1, v2) = Split::split(data);
        Ok((
            L::converter().deserialize(v1)?,
            R::converter().deserialize(v2)?,
        ))
    }

    fn serialize(
        &self,
        value: Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
        Ok(Concat::concat(
            L::converter().serialize(value.0)?,
            R::converter().serialize(value.1)?,
        ))
    }
}

impl<L: Value, R: Value> ConverterInstance for TupleConverter<L, R>
where
    TagsOfValueView<L>: MergeList<TagsOfValueView<R>>,
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    const INSTANCE: Self = TupleConverter(PhantomData);
}
