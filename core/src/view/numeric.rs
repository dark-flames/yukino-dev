use crate::entity::converter::DataConverter;
use crate::entity::FieldView;
use crate::expr::View;
use crate::query::computation::Computation;
use crate::query::optimizer::QueryOptimizer;
use crate::resolver::field_resolve_cells::numeric::*;

macro_rules! implement_view_of {
    ($ty: ty, $name: ident, $converter: ty) => {
        pub struct $name {
            converter: $converter
        }

        impl View for $name {
            type Output = $ty;

            fn computation<'f>(&self) -> Computation<'f, Self::Output> {
                Computation::create(self.converter.field_value_converter())
            }

            fn optimizer(&self) -> Box<dyn QueryOptimizer> {
                todo!("add column in select list")
            }
        }

        impl FieldView for $name {
            type Type = $ty;
        }
    }
}

implement_view_of!(i16, ShortFieldView, ShortDataConverter);
implement_view_of!(u16, UnsignedShortFieldView, UnsignedShortDataConverter);
implement_view_of!(i32, IntFieldView, IntDataConverter);
implement_view_of!(u32, UnsignedIntFieldView, UnsignedIntDataConverter);
implement_view_of!(i64, LongFieldView, LongDataConverter);
implement_view_of!(u64, UnsignedLongFieldView, UnsignedLongDataConverter);


