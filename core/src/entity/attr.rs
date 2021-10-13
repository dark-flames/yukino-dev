use annotation_rs::{Annotation, AnnotationEnumValue};
use std::collections::HashMap;

#[derive(Annotation, Clone)]
#[mod_path = "yukino::entity::attr"]
pub struct Entity {
    pub name: Option<String>,
    pub indexes: Option<HashMap<String, Index>>,
}

#[derive(Annotation, Clone)]
#[mod_path = "yukino::entity::attr"]
pub struct Index {
    pub fields: Vec<String>,
    #[field(enum_value = true, default = "b_tree")]
    pub method: IndexMethod,
    #[field(default = false)]
    pub unique: bool,
}

#[derive(AnnotationEnumValue, Copy, Clone, Debug)]
#[mod_path = "yukino::entity::attr"]
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
#[mod_path = "yukino::entity::attr"]
pub struct ID;

#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Field {
    pub name: Option<String>,
    #[field(default = false)]
    pub unique: bool,
    #[field(default = false)]
    pub auto_increase: bool,
    pub options: Option<HashMap<String, String>>,
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
