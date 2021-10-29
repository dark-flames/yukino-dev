use crate::interface::converter::DataConverter;
use crate::interface::FieldView;
use crate::query::computation::Computation;
use crate::query::optimizer::{QueryOptimizer, SelectAppendOptimizer};
use crate::view::View;

macro_rules! implement_view_of {
    ($ty: ty, $name: ident, $converter: ty) => {
        pub struct $name {
            converter: &'static dyn DataConverter<Output = $ty>,
        }

        impl View for $name {
            type Output = $ty;

            fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                Computation::create(self.converter.field_value_converter())
            }

            fn optimizer(&self) -> Box<dyn QueryOptimizer> {
                let mut optimizer = SelectAppendOptimizer::create();
                optimizer.append_by_columns(self.converter.get_columns());

                optimizer
            }
        }

        impl FieldView for $name {
            type ConverterType = $ty;

            fn create(
                converter: &'static dyn DataConverter<Output = Self::ConverterType>,
            ) -> Box<Self>
            where
                Self: Sized,
            {
                Box::new($name { converter })
            }

            fn get_converter(&self) -> &'static dyn DataConverter<Output = Self::ConverterType> {
                self.converter
            }
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
implement_view_of!(String, StringFieldView, StringDataConverter);
implement_view_of!(char, CharFieldView, CharDataConverter);

pub struct OptionalFieldWrapper<V: FieldView> {
    view: Box<V>,
}

impl<V: FieldView> View for OptionalFieldWrapper<V> {
    type Output = Option<V::ConverterType>;

    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        Computation::create(self.view.get_converter().nullable_field_value_converter())
    }

    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        self.view.optimizer()
    }
}

impl<V: FieldView> FieldView for OptionalFieldWrapper<V> {
    type ConverterType = V::ConverterType;

    fn create(converter: &'static dyn DataConverter<Output=Self::ConverterType>) -> Box<Self>
        where
            Self: Sized,
    {
        Box::new(OptionalFieldWrapper {
            view: V::create(converter),
        })
    }

    fn get_converter(&self) -> &'static dyn DataConverter<Output=Self::ConverterType> {
        self.view.get_converter()
    }
}
