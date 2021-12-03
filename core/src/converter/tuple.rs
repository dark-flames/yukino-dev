use std::marker::PhantomData;
use std::ops::{Add, Sub};

use generic_array::{
    GenericArray,
    sequence::{Concat, Split},
    typenum::Sum,
};

use query_builder::DatabaseValue;

use crate::converter::{Converter, ConverterInstance, ConvertResult, Deserializer};
use crate::view::{Value, ValueCount, ValueCountOf};

pub struct TupleConverter<L: Value, R: Value>(PhantomData<(L, R)>);

impl<L: Value, R: Value> Converter for TupleConverter<L, R>
where
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

    fn deserializer(&self) -> Deserializer<Self::Output> {
        Box::new(|v| {
            let (v1, v2) = Split::split(v);
            Ok((
                (*L::converter().deserializer())(v1)?,
                (*R::converter().deserializer())(v2)?,
            ))
        })
    }

    fn serialize(
        &self,
        value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
        Ok(Concat::concat(
            L::converter().serialize(&value.0)?,
            R::converter().serialize(&value.1)?,
        ))
    }
}

impl<L: Value, R: Value> ConverterInstance for TupleConverter<L, R>
where
    ValueCountOf<L>: Add<ValueCountOf<R>>,
    Sum<ValueCountOf<L>, ValueCountOf<R>>:
        ValueCount + Sub<ValueCountOf<L>, Output = ValueCountOf<R>>,
{
    const INSTANCE: Self = TupleConverter(PhantomData);
}
