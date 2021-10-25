use std::any::type_name;

use heck::SnakeCase;
use iroha::ToTokens;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Field as SynField, Type};

use crate::db::ty::{DatabaseType, DatabaseValue, ValuePack};
use crate::interface::attr::{Field, FieldAttribute, IndexMethod};
use crate::interface::converter::DataConverter;
use crate::interface::def::{
    ColumnDefinition, DefinitionType, FieldDefinition, IndexDefinition, IndexType,
};
use crate::err::DataConvertError;
use crate::err::{CliResult, RuntimeResult};
use crate::err::{ResolveError, YukinoError};
use crate::resolver::field::{
    FieldPath, FieldResolveResult, FieldResolverCell, FieldResolverCellBox, FieldResolverSeed,
    ResolvedField,
};
use crate::resolver::path::{FileTypePathResolver, TypeMatchResult};
use crate::view::numeric::{
    DoubleFieldView, FloatFieldView, IntFieldView, LongFieldView, ShortFieldView,
    UnsignedIntFieldView, UnsignedLongFieldView, UnsignedShortFieldView,
};

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
                    .unwrap_or(Field {
                        name: Option::None,
                        unique: false,
                        auto_increase: false,
                    });

                Ok(NumericFieldResolverCell {
                    ty,
                    optional,
                    primary: attrs.iter().any(|attr| matches!(attr, FieldAttribute::ID(_))),
                    auto_increase: field.auto_increase,
                    unique: field.unique,
                    column: field.name.unwrap_or(field_name).to_snake_case(),
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
            converter: self.ty.converter(self.column.clone()),
            view: self.ty.view(self.column.clone()),
            primary: self.primary,
            entities: vec![],
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

    pub fn converter(&self, column_name: String) -> TokenStream {
        match self {
            NumericType::Short => ShortDataConverter { column_name }.to_token_stream(),
            NumericType::UnsignedShort => {
                UnsignedShortDataConverter { column_name }.to_token_stream()
            }
            NumericType::Int => IntDataConverter { column_name }.to_token_stream(),
            NumericType::UnsignedInt => UnsignedIntDataConverter { column_name }.to_token_stream(),
            NumericType::Long => LongDataConverter { column_name }.to_token_stream(),
            NumericType::UnsignedLong => {
                UnsignedLongDataConverter { column_name }.to_token_stream()
            }
            NumericType::Float => FloatDataConverter { column_name }.to_token_stream(),
            NumericType::Double => DoubleDataConverter { column_name }.to_token_stream(),
        }
    }

    pub fn view(&self, column_name: String) -> TokenStream {
        match self {
            NumericType::Short => {
                ShortFieldView::new(ShortDataConverter { column_name }).to_token_stream()
            }
            NumericType::UnsignedShort => {
                UnsignedShortFieldView::new(UnsignedShortDataConverter { column_name })
                    .to_token_stream()
            }
            NumericType::Int => {
                IntFieldView::new(IntDataConverter { column_name }).to_token_stream()
            }
            NumericType::UnsignedInt => {
                UnsignedIntFieldView::new(UnsignedIntDataConverter { column_name })
                    .to_token_stream()
            }
            NumericType::Long => {
                LongFieldView::new(LongDataConverter { column_name }).to_token_stream()
            }
            NumericType::UnsignedLong => {
                UnsignedLongFieldView::new(UnsignedLongDataConverter { column_name })
                    .to_token_stream()
            }
            NumericType::Float => {
                FloatFieldView::new(FloatDataConverter { column_name }).to_token_stream()
            }
            NumericType::Double => {
                DoubleFieldView::new(DoubleDataConverter { column_name }).to_token_stream()
            }
        }
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

macro_rules! converter_of {
    ($field_type:ty, $name:ident, $enum_variant:ident) => {
        #[derive(ToTokens, Clone)]
        #[Iroha(mod_path = "yukino::resolver::field_resolve_cells::numeric")]
        pub struct $name {
            column_name: String,
        }

        impl DataConverter for $name {
            type FieldType = $field_type;
            fn field_value_converter(
                &self,
            ) -> Box<dyn Fn(&ValuePack) -> RuntimeResult<Self::FieldType>> {
                let column_name = self.column_name.clone();
                Box::new(move |values| {
                    values
                        .get(column_name.as_str())
                        .map(|data| match data {
                            DatabaseValue::$enum_variant(data) => Ok(data.clone()),
                            _ => Err(DataConvertError::UnexpectedValueType(column_name.clone())
                                .as_runtime_err()),
                        })
                        .ok_or_else(|| {
                            DataConvertError::ColumnDataNotFound(column_name.clone())
                                .as_runtime_err()
                        })?
                })
            }

            fn to_database_values_by_ref(
                &self,
                value: &Self::FieldType,
            ) -> RuntimeResult<ValuePack> {
                Ok([(
                    self.column_name.clone(),
                    DatabaseValue::$enum_variant(value.clone()),
                )]
                .into_iter()
                .collect())
            }
        }
    };
}

converter_of!(i16, ShortDataConverter, SmallInteger);
converter_of!(u16, UnsignedShortDataConverter, UnsignedSmallInteger);
converter_of!(i32, IntDataConverter, Integer);
converter_of!(u32, UnsignedIntDataConverter, UnsignedInteger);
converter_of!(i64, LongDataConverter, BigInteger);
converter_of!(u64, UnsignedLongDataConverter, UnsignedBigInteger);
converter_of!(f32, FloatDataConverter, Float);
converter_of!(f64, DoubleDataConverter, Double);
