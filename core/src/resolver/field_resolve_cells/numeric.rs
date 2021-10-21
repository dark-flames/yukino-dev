use crate::db::ty::DatabaseType;
use crate::entity::attr::{Field, FieldAttribute, IndexMethod};
use crate::entity::def::{
    ColumnDefinition, DefinitionType, FieldDefinition, IndexDefinition, IndexType,
};
use crate::err::{ResolveError, YukinoError};
use crate::resolver::entry::CliResult;
use crate::resolver::field::{
    FieldPath, FieldResolveResult, FieldResolverCell, FieldResolverCellBox, FieldResolverSeed,
    ResolvedField,
};
use crate::resolver::path::{FileTypePathResolver, TypeMatchResult};
use heck::SnakeCase;
use std::any::type_name;
use syn::spanned::Spanned;
use syn::{Field as SynField, Type};

pub struct NumericFieldResolverSeed {}

pub struct NumericFieldResolverCell {
    ty: NumericType,
    optional: bool,
    primary: bool,
    auto_increase: bool,
    unique: bool,
    column: String,
}

#[derive(Copy, Clone)]
pub enum NumericType {
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    Float,
    Double,
}

impl FieldResolverSeed for NumericFieldResolverSeed {
    fn match_field(
        &self,
        field: &SynField,
        type_resolver: &FileTypePathResolver,
    ) -> CliResult<Option<FieldResolverCellBox>> {
        NumericType::from_ty(&field.ty, type_resolver)
            .map(|(ty, optional)| {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let attrs: Vec<FieldAttribute> = field
                    .attrs
                    .iter()
                    .map(|a| {
                        FieldAttribute::from_attr(a).map_err(|e| {
                            ResolveError::FieldParseError(field_name.clone(), e.to_string())
                                .as_cli_err(Some(field.span()))
                        })
                    })
                    .collect::<CliResult<Vec<_>>>()?;

                let field = attrs
                    .iter()
                    .filter_map(|attr| {
                        if let FieldAttribute::Field(field) = attr {
                            Some(field.clone())
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or_else(|| Field {
                        name: Option::None,
                        unique: false,
                        auto_increase: false,
                    });

                Ok(NumericFieldResolverCell {
                    ty,
                    optional,
                    primary: attrs.iter().any(|attr| {
                        if let FieldAttribute::ID(_) = attr {
                            true
                        } else {
                            false
                        }
                    }),
                    auto_increase: field.auto_increase,
                    unique: field.unique,
                    column: field.name.unwrap_or_else(|| field_name).to_snake_case(),
                }
                .wrap())
            })
            .map_or(Ok(None), |r| r.map(Some))
    }
}

impl FieldResolverCell for NumericFieldResolverCell {
    fn resolve(
        &self,
        _type_resolver: &FileTypePathResolver,
        field_path: FieldPath,
    ) -> CliResult<FieldResolveResult> {
        Ok(FieldResolveResult::Finished(ResolvedField {
            path: field_path.clone(),
            definition: FieldDefinition {
                name: field_path.field_name.clone(),
                ty: self.ty.to_string(),
                auto_increase: self.auto_increase,
                definition_ty: DefinitionType::Normal,
                columns: [(
                    self.column.clone(),
                    ColumnDefinition {
                        name: self.column.clone(),
                        ty: DatabaseType::from(&self.ty),
                        nullable: self.optional,
                        auto_increase: self.auto_increase,
                    },
                )]
                .into_iter()
                .collect(),
                identity_columns: vec![self.column.clone()],
                association: Option::None,
                indexes: if self.unique {
                    vec![IndexDefinition {
                        name: format!("_{}_unique", field_path.field_name),
                        fields: vec![self.column.clone()],
                        ty: IndexType::Unique,
                        method: IndexMethod::BTree,
                    }]
                } else {
                    vec![]
                },
            },
            primary: self.primary,
        }))
    }
}

impl ToString for NumericType {
    fn to_string(&self) -> String {
        match self {
            NumericType::Short => type_name::<i16>(),
            NumericType::UnsignedShort => type_name::<u16>(),
            NumericType::Int => type_name::<i32>(),
            NumericType::UnsignedInt => type_name::<u32>(),
            NumericType::Long => type_name::<i64>(),
            NumericType::UnsignedLong => type_name::<u64>(),
            NumericType::Float => type_name::<f32>(),
            NumericType::Double => type_name::<f64>(),
        }
        .to_string()
    }
}

impl NumericType {
    pub fn from_ty(ty: &Type, resolver: &FileTypePathResolver) -> Option<(Self, bool)> {
        let branch: [(NumericType, Box<dyn Fn() -> TypeMatchResult>); 8] = [
            (
                NumericType::Short,
                Box::new(|| resolver.match_ty::<i16>(ty)),
            ),
            (
                NumericType::UnsignedShort,
                Box::new(|| resolver.match_ty::<u16>(ty)),
            ),
            (NumericType::Int, Box::new(|| resolver.match_ty::<i32>(ty))),
            (
                NumericType::UnsignedInt,
                Box::new(|| resolver.match_ty::<u32>(ty)),
            ),
            (NumericType::Long, Box::new(|| resolver.match_ty::<i64>(ty))),
            (
                NumericType::UnsignedLong,
                Box::new(|| resolver.match_ty::<u64>(ty)),
            ),
            (
                NumericType::Float,
                Box::new(|| resolver.match_ty::<f32>(ty)),
            ),
            (
                NumericType::Double,
                Box::new(|| resolver.match_ty::<f64>(ty)),
            ),
        ];
        branch
            .iter()
            .map(|(numeric_ty, f)| match (*f)() {
                TypeMatchResult::Match => Some((*numeric_ty, false)),
                TypeMatchResult::InOption => Some((*numeric_ty, true)),
                TypeMatchResult::Mismatch => None,
            })
            .find(|r| r.is_some())
            .flatten()
    }
}

impl From<&NumericType> for DatabaseType {
    fn from(ty: &NumericType) -> Self {
        match ty {
            NumericType::Short => DatabaseType::SmallInteger,
            NumericType::UnsignedShort => DatabaseType::UnsignedSmallInteger,
            NumericType::Int => DatabaseType::Integer,
            NumericType::UnsignedInt => DatabaseType::UnsignedInteger,
            NumericType::Long => DatabaseType::BigInteger,
            NumericType::UnsignedLong => DatabaseType::UnsignedBigInteger,
            NumericType::Float => DatabaseType::Float,
            NumericType::Double => DatabaseType::Double,
        }
    }
}
