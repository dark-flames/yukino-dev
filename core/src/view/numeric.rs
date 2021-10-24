use crate::entity::converter::DataConverter;
use crate::entity::FieldView;
use crate::expr::View;
use crate::query::calc::Computation;
use crate::query::optimizer::QueryOptimizer;
use crate::resolver::field_resolve_cells::numeric::ShortDataConverter;

pub struct ShortFieldView {
    _column_name: String,
    converter: ShortDataConverter
}

impl View for ShortFieldView {
    type Output = i16;

    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        Computation::create(self.converter.field_value_converter())
    }

    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        todo!("add column in select list")
    }
}

impl FieldView for ShortFieldView {
    type Type = i16;
}

