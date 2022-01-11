use generic_array::{arr, GenericArray};

use query_builder::DatabaseValue;

use crate::converter::{Converter, ConverterInstance, ConvertResult};
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

    fn deserialize(
        &self,
        _data: GenericArray<DatabaseValue, ValueCountOf<Self::Output>>,
    ) -> ConvertResult<Self::Output> {
        Ok(())
    }

    fn serialize(
        &self,
        _value: Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
        Ok(arr![DatabaseValue;])
    }
}

impl ConverterInstance for UnitConverter {
    const INSTANCE: Self = UnitConverter;
}
