use crate::converter::{ConvertResult, Converter, ConverterInstance, Deserializer};
use crate::view::{Value, ValueCount};
use generic_array::{
    sequence::{Concat, Split},
    GenericArray,
};
use query_builder::DatabaseValue;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

pub struct TupleConverter<V0: Value, V1: Value>(PhantomData<(V0, V1)>);

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>>
Converter for TupleConverter<V0, V1>
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    type Output = (V0, V1);

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
                (*V0::converter().deserializer())(v1)?,
                (*V1::converter().deserializer())(v2)?,
            ))
        })
    }

    fn serialize(
        &self,
        value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>> {
        Ok(Concat::concat(
            V0::converter().serialize(&value.0)?,
            V1::converter().serialize(&value.1)?,
        ))
    }
}

impl<V0: Value, V1: Value, OL: ValueCount + Sub<<V0 as Value>::L, Output=<V1 as Value>::L>>
ConverterInstance for TupleConverter<V0, V1>
    where
        <V0 as Value>::L: Add<<V1 as Value>::L, Output=OL>,
{
    const INSTANCE: Self = TupleConverter(PhantomData);
}
