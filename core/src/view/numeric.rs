use crate::interface::converter::DataConverter;
use crate::interface::FieldView;
use crate::query::computation::Computation;
use crate::query::optimizer::{QueryOptimizer, SelectAppendOptimizer};
use crate::view::View;
use iroha::ToTokens;

macro_rules! implement_view_of {
    ($ty: ty, $name: ident, $converter: ty) => {
        #[derive(ToTokens)]
        #[Iroha(mod_path = "yukino::view::numeric")]
        pub struct $name {
            converter: Box<dyn DataConverter<FieldType = $ty>>,
        }

        impl View for $name {
            type Output = $ty;

            fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                Computation::create(self.converter.field_value_converter())
            }

            fn optimizer(&self) -> Box<dyn QueryOptimizer> {
                let mut optimizer: SelectAppendOptimizer = Default::default();
                optimizer.append_by_columns(self.converter.get_columns());

                Box::new(optimizer)
            }
        }

        impl FieldView for $name {
            type Type = $ty;
        }
    };
}

implement_view_of!(i16, ShortFieldView, ShortDataConverter);
implement_view_of!(u16, UnsignedShortFieldView, UnsignedShortDataConverter);
implement_view_of!(i32, IntFieldView, IntDataConverter);
implement_view_of!(u32, UnsignedIntFieldView, UnsignedIntDataConverter);
implement_view_of!(i64, LongFieldView, LongDataConverter);
implement_view_of!(u64, UnsignedLongFieldView, UnsignedLongDataConverter);
implement_view_of!(f32, FloatFieldView, FloatDataConverter);
implement_view_of!(f64, DoubleFieldView, DoubleDataConverter);
