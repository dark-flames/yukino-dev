use crate::resolver::entry::CliResult;
use crate::resolver::field::{
    FieldPath, FieldResolveResult, FieldResolverCell, FieldResolverCellBox, FieldResolverSeed,
};
use crate::resolver::path::FileTypePathResolver;
use syn::Field;

pub struct NumericFieldResolverSeed {}

pub struct NumericFieldResolverCell {}

impl FieldResolverSeed for NumericFieldResolverSeed {
    fn match_field(
        &self,
        _field: &Field,
        _type_resolver: &FileTypePathResolver,
    ) -> Option<FieldResolverCellBox> {
        todo!()
    }
}

impl FieldResolverCell for NumericFieldResolverCell {
    fn resolve(
        &self,
        _type_resolver: &FileTypePathResolver,
        _field_path: FieldPath,
    ) -> CliResult<FieldResolveResult> {
        todo!()
    }
}
