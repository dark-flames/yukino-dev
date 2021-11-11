use annotation_rs::{Annotation, AnnotationEnumValue, AnnotationStructure};
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Attribute, Error};

#[derive(Annotation, Clone)]
#[mod_path = "yukino::interface::attr"]
pub struct Entity {
    pub name: Option<String>,
    pub indexes: Option<HashMap<String, Index>>,
}

#[derive(Annotation, Clone)]
#[mod_path = "yukino::interface::attr"]
pub struct Index {
    pub fields: Vec<String>,
    #[field(enum_value = true, default = "b_tree")]
    pub method: IndexMethod,
    #[field(default = false)]
    pub unique: bool,
}

#[derive(AnnotationEnumValue, Copy, Clone, Debug)]
#[mod_path = "yukino::interface::attr"]
pub enum IndexMethod {
    BTree,
    #[cfg(any(feature = "mysql", feature = "postgre-sql"))]
    Hash,
    #[cfg(any(feature = "postgre-sql"))]
    Gin,
    #[cfg(any(feature = "postgre-sql"))]
    #[variant_value("sp_gin")]
    SPGin,
    #[cfg(any(feature = "postgre-sql"))]
    Gist,
    #[cfg(any(feature = "postgre-sql"))]
    Brin,
}

#[derive(Annotation, Clone)]
#[mod_path = "yukino::interface::attr"]
pub struct ID;

#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Field {
    pub name: Option<String>,
    #[field(default = false)]
    pub unique: bool,
    #[field(default = false)]
    pub auto_increase: bool,
}

#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Association {
    pub mapped_by: Option<Vec<String>>,
    #[field(default = false)]
    pub unique: bool,
}

#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct InverseAssociation {
    pub inversed_by: String,
}

pub enum FieldAttribute {
    ID(ID),
    Field(Field),
    Association(Association),
    InverseAssociation(InverseAssociation),
}

impl FieldAttribute {
    pub fn from_attr(attr: &Attribute) -> Result<Self, Error> {
        if attr.path == ID::get_path() {
            Ok(FieldAttribute::ID(ID))
        } else if attr.path == Field::get_path() {
            Ok(FieldAttribute::Field(Field::from_meta(
                &attr.parse_meta()?,
            )?))
        } else if attr.path == Association::get_path() {
            Ok(FieldAttribute::Association(Association::from_meta(
                &attr.parse_meta()?,
            )?))
        } else if attr.path == InverseAssociation::get_path() {
            Ok(FieldAttribute::InverseAssociation(
                InverseAssociation::from_meta(&attr.parse_meta()?)?,
            ))
        } else {
            Err(Error::new_spanned(
                attr,
                format!("Unexpected attribute: {}", attr.path.to_token_stream()),
            ))
        }
    }
}
