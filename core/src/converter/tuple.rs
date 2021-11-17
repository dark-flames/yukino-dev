use crate::converter::{ConvertResult, Converter, ConverterInstance, Deserializer};
use crate::view::{Value, ValueCount};
use generic_array::{
    sequence::{Concat, Split},
    GenericArray,
};
use query_builder::DatabaseValue;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

pub struct TupleConverter<L: Value, R: Value>(PhantomData<(L, R)>);

impl<L: Value, R: Value, OL: ValueCount + Sub<<L as Value>::L, Output=<R as Value>::L>> Converter
for TupleConverter<L, R>
    where
        <L as Value>::L: Add<<R as Value>::L, Output=OL>,
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
    ) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>> {
        Ok(Concat::concat(
            L::converter().serialize(&value.0)?,
            R::converter().serialize(&value.1)?,
        ))
    }
}

impl<L: Value, R: Value, OL: ValueCount + Sub<<L as Value>::L, Output=<R as Value>::L>>
ConverterInstance for TupleConverter<L, R>
    where
        <L as Value>::L: Add<<R as Value>::L, Output=OL>,
{
    const INSTANCE: Self = TupleConverter(PhantomData);
}
