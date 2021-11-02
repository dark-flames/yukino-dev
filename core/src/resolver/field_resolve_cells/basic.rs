use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_str, Field as SynField, Type};

use crate::db::ty::DatabaseType;
use crate::err::CliResult;
use crate::err::{ResolveError, YukinoError};
use crate::interface::attr::{Field, FieldAttribute, IndexMethod};
use crate::interface::def::{
    ColumnDefinition, DefinitionType, FieldDefinition, IndexDefinition, IndexType,
};
use crate::resolver::field::{
    FieldPath, FieldResolveResult, FieldResolverCell, FieldResolverCellBox, FieldResolverSeed,
    FieldResolverSeedBox, ResolvedField,
};
use crate::resolver::path::{FileTypePathResolver, TypeMatchResult};

pub struct BasicFieldResolverSeed();

pub struct BasicFieldResolverCell {
    ty: FieldType,
    optional: bool,
    primary: bool,
    auto_increase: bool,
    unique: bool,
    column: String,
}

#[derive(Copy, Clone)]
pub enum FieldType {
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    Float,
    Double,
    String,
    Char,
}

impl FieldResolverSeed for BasicFieldResolverSeed {
    fn instance() -> FieldResolverSeedBox
    where
        Self: Sized,
    {
        Box::new(BasicFieldResolverSeed())
    }

    fn match_field(
        &self,
        field: &SynField,
        type_resolver: &FileTypePathResolver,
    ) -> CliResult<Option<FieldResolverCellBox>> {
        FieldType::from_ty(&field.ty, type_resolver)
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

                Ok(BasicFieldResolverCell {
                    ty,
                    optional,
                    primary: attrs
                        .iter()
                        .any(|attr| matches!(attr, FieldAttribute::ID(_))),
                    auto_increase: field.auto_increase,
                    unique: field.unique,
                    column: field.name.unwrap_or(field_name).to_snake_case(),
                }
                .wrap())
            })
            .map_or(Ok(None), |r| r.map(Some))
    }
}

impl FieldResolverCell for BasicFieldResolverCell {
    fn resolve(
        &self,
        _type_resolver: &FileTypePathResolver,
        field_path: FieldPath,
    ) -> CliResult<FieldResolveResult> {
        Ok(FieldResolveResult::Finished(Box::new(ResolvedField {
            path: field_path.clone(),
            definition: FieldDefinition {
                name: field_path.field_name.clone(),
                ty: self.ty.field_ty(self.optional).to_string(),
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
            converter: self.ty.converter(self.optional),
            converter_type: self.ty.converter_ty(self.optional),
            converter_param_count: 1,
            ty: self.ty.field_ty(self.optional),
            view: self.ty.view(&self.column),
            marker_name: format_ident!("{}", field_path.field_name.to_snake_case()),
            primary: self.primary,
            entities: vec![],
        })))
    }
}

impl FieldType {
    pub fn from_ty(ty: &Type, resolver: &FileTypePathResolver) -> Option<(Self, bool)> {
        let branch: [(FieldType, Box<dyn Fn() -> TypeMatchResult>); 10] = [
            (FieldType::Short, Box::new(|| resolver.match_ty::<i16>(ty))),
            (
                FieldType::UnsignedShort,
                Box::new(|| resolver.match_ty::<u16>(ty)),
            ),
            (FieldType::Int, Box::new(|| resolver.match_ty::<i32>(ty))),
            (
                FieldType::UnsignedInt,
                Box::new(|| resolver.match_ty::<u32>(ty)),
            ),
            (FieldType::Long, Box::new(|| resolver.match_ty::<i64>(ty))),
            (
                FieldType::UnsignedLong,
                Box::new(|| resolver.match_ty::<u64>(ty)),
            ),
            (FieldType::Float, Box::new(|| resolver.match_ty::<f32>(ty))),
            (FieldType::Double, Box::new(|| resolver.match_ty::<f64>(ty))),
            (
                FieldType::String,
                Box::new(|| {
                    let str = parse_str("String").unwrap();
                    resolver.match_ty_by_param(ty, &str)
                }),
            ),
            (FieldType::Char, Box::new(|| resolver.match_ty::<char>(ty))),
        ];
        branch
            .iter()
            .map(|(field_type, f)| match (*f)() {
                TypeMatchResult::Match => Some((*field_type, false)),
                TypeMatchResult::InOption => Some((*field_type, true)),
                TypeMatchResult::Mismatch => None,
            })
            .find(|r| r.is_some())
            .flatten()
    }

    pub fn converter(&self, optional: bool) -> TokenStream {
        let ty = self.converter_ty(optional);

        quote! {
            #ty()
        }
    }

    pub fn converter_ty(&self, optional: bool) -> TokenStream {
        let prefix = if optional { "Optional" } else { "" };
        let name = match self {
            FieldType::Short => format_ident!("{}ShortConverter", prefix),
            FieldType::UnsignedShort => format_ident!("{}UnsignedShortConverter", prefix),
            FieldType::Int => format_ident!("{}IntConverter", prefix),
            FieldType::UnsignedInt => format_ident!("{}UnsignedIntConverter", prefix),
            FieldType::Long => format_ident!("{}LongConverter", prefix),
            FieldType::UnsignedLong => format_ident!("{}UnsignedLongConverter", prefix),
            FieldType::Float => format_ident!("{}FloatConverter", prefix),
            FieldType::Double => format_ident!("{}DoubleConverter", prefix),
            FieldType::String => format_ident!("{}StringConverter", prefix),
            FieldType::Char => format_ident!("{}CharConverter", prefix),
        };

        quote! {
            yukino::converter::basic::#name
        }
    }

    pub fn view(&self, column: &str) -> TokenStream {
        let database_ty = DatabaseType::from(self);

        quote! {
            Box::new(ViewNode::Expr(ExprView::create(vec![alias.create_ident_expr(
                #column, #database_ty
            )])))
        }
    }

    pub fn field_ty(&self, optional: bool) -> TokenStream {
        let inside: TokenStream = parse_str(match self {
            FieldType::Short => "i16",
            FieldType::UnsignedShort => "u16",
            FieldType::Int => "i32",
            FieldType::UnsignedInt => "u32",
            FieldType::Long => "i64",
            FieldType::UnsignedLong => "u64",
            FieldType::Float => "f32",
            FieldType::Double => "f64",
            FieldType::String => "String",
            FieldType::Char => "char",
        })
        .unwrap();

        if optional {
            quote! {
                Option<#inside>
            }
        } else {
            inside
        }
    }
}

impl From<&FieldType> for DatabaseType {
    fn from(ty: &FieldType) -> Self {
        match ty {
            FieldType::Short => DatabaseType::SmallInteger,
            FieldType::UnsignedShort => DatabaseType::UnsignedSmallInteger,
            FieldType::Int => DatabaseType::Integer,
            FieldType::UnsignedInt => DatabaseType::UnsignedInteger,
            FieldType::Long => DatabaseType::BigInteger,
            FieldType::UnsignedLong => DatabaseType::UnsignedBigInteger,
            FieldType::Float => DatabaseType::Float,
            FieldType::Double => DatabaseType::Double,
            FieldType::String => DatabaseType::String,
            FieldType::Char => DatabaseType::Character,
        }
    }
}
