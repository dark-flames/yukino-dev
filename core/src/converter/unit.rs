use generic_array::{arr, GenericArray};

use query_builder::DatabaseValue;

use crate::converter::{Converter, ConverterInstance, ConvertResult, Deserializer};
use crate::view::ValueCountOf;

pub struct UnitConverter;

impl Converter for UnitConverter {
    type Output = ();

    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &<Self as ConverterInstance>::INSTANCE
    }

    fn deserializer(&self) -> Deserializer<Self::Output> {
        Box::new(|_v| Ok(()))
    }

    fn serialize(
        &self,
        _value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
        Ok(arr![DatabaseValue;])
    }
}

impl ConverterInstance for UnitConverter {
    const INSTANCE: Self = UnitConverter;
}
