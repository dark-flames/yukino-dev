use crate::converter::{ConvertResult, Converter, ConverterInstance};
use crate::db::ty::DatabaseValue;
use crate::view::Value;
use std::marker::PhantomData;

pub struct TupleConverter<L: Value, R: Value>(PhantomData<L>, PhantomData<R>);

unsafe impl<L: Value, R: Value> Sync for TupleConverter<L, R> {}

impl<L: Value, R: Value> Converter for TupleConverter<L, R> {
    type Output = (L, R);

    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &Self::INSTANCE
    }

    fn param_count(&self) -> usize {
        L::converter().param_count() + R::converter().param_count()
    }

    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> ConvertResult<Self::Output>> {
        let l_count = L::converter().param_count();
        let total_count = self.param_count();
        Box::new(move |v| {
            Ok((
                (*L::converter().deserializer())(&v[0..l_count])?,
                (*R::converter().deserializer())(&v[l_count..total_count])?,
            ))
        })
    }

    fn serialize(&self, value: &Self::Output) -> ConvertResult<Vec<DatabaseValue>> {
        let (l, r) = value;
        let mut result = L::converter().serialize(l)?;
        result.extend(R::converter().serialize(r)?);

        Ok(result)
    }
}

impl<L: Value, R: Value> ConverterInstance for TupleConverter<L, R> {
    const INSTANCE: Self = TupleConverter(PhantomData, PhantomData);
}
