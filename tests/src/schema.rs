use yukino::converter::ConverterRef;
use yukino::converter::{ConvertResult, Converter};
use yukino::db::ty::DatabaseValue;
use yukino::err::{RuntimeResult, YukinoError};
use yukino::interface::Entity;
use yukino::interface::EntityView;
use yukino::query::{Alias, Expr};
use yukino::view::Value;
use yukino::view::{Computation, ExprView, View, ViewBox, ViewNode};
#[derive(Clone, Debug)]
pub struct Basic {
    pub character: char,
    pub double: f64,
    pub float: f32,
    pub id: u32,
    pub int: i32,
    pub long: i64,
    pub optional: Option<u32>,
    pub short: i16,
    pub string: String,
    pub u_int: u32,
    pub u_long: u64,
    pub u_short: u16,
}
#[derive(Debug)]
pub struct BasicView {
    pub character: ViewBox<char>,
    pub double: ViewBox<f64>,
    pub float: ViewBox<f32>,
    pub id: ViewBox<u32>,
    pub int: ViewBox<i32>,
    pub long: ViewBox<i64>,
    pub optional: ViewBox<Option<u32>>,
    pub short: ViewBox<i16>,
    pub string: ViewBox<String>,
    pub u_int: ViewBox<u32>,
    pub u_long: ViewBox<u64>,
    pub u_short: ViewBox<u16>,
}
unsafe impl Sync for BasicView {}
impl Clone for BasicView {
    fn clone(&self) -> Self {
        BasicView {
            character: self.character.clone(),
            double: self.double.clone(),
            float: self.float.clone(),
            id: self.id.clone(),
            int: self.int.clone(),
            long: self.long.clone(),
            optional: self.optional.clone(),
            short: self.short.clone(),
            string: self.string.clone(),
            u_int: self.u_int.clone(),
            u_long: self.u_long.clone(),
            u_short: self.u_short.clone(),
        }
    }
}
impl Computation for BasicView {
    type Output = Basic;
    fn eval(&self, v: &[&DatabaseValue]) -> RuntimeResult<Self::Output> {
        (*Basic::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}
impl View<Basic> for BasicView {
    fn view_node(&self) -> ViewNode<Basic> {
        ViewNode::Expr(ExprView::create(self.collect_expr()))
    }
    fn collect_expr(&self) -> Vec<Expr> {
        let mut exprs = vec![];
        exprs.extend(self.character.collect_expr());
        exprs.extend(self.double.collect_expr());
        exprs.extend(self.float.collect_expr());
        exprs.extend(self.id.collect_expr());
        exprs.extend(self.int.collect_expr());
        exprs.extend(self.long.collect_expr());
        exprs.extend(self.optional.collect_expr());
        exprs.extend(self.short.collect_expr());
        exprs.extend(self.string.collect_expr());
        exprs.extend(self.u_int.collect_expr());
        exprs.extend(self.u_long.collect_expr());
        exprs.extend(self.u_short.collect_expr());
        exprs
    }
    fn clone(&self) -> ViewBox<Basic> {
        Box::new(Clone::clone(self))
    }
}
impl EntityView for BasicView {
    type Entity = Basic;
    fn pure(alias: Alias) -> Self
    where
        Self: Sized,
    {
        BasicView {
            character: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("character", yukino::db::ty::DatabaseType::Character)
            ]))),
            double: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("double", yukino::db::ty::DatabaseType::Double)
            ]))),
            float: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("float", yukino::db::ty::DatabaseType::Float)
            ]))),
            id: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "id",
                    yukino::db::ty::DatabaseType::UnsignedInteger,
                )]))),
            int: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("int", yukino::db::ty::DatabaseType::Integer)
            ]))),
            long: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("long", yukino::db::ty::DatabaseType::BigInteger)
            ]))),
            optional: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "optional",
                    yukino::db::ty::DatabaseType::UnsignedInteger,
                )]))),
            short: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "short",
                    yukino::db::ty::DatabaseType::SmallInteger,
                )]))),
            string: Box::new(ViewNode::Expr(ExprView::create(vec![
                alias.create_ident_expr("string", yukino::db::ty::DatabaseType::String)
            ]))),
            u_int: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "u_int",
                    yukino::db::ty::DatabaseType::UnsignedInteger,
                )]))),
            u_long: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "u_long",
                    yukino::db::ty::DatabaseType::UnsignedBigInteger,
                )]))),
            u_short: Box::new(ViewNode::Expr(ExprView::create(vec![alias
                .create_ident_expr(
                    "u_short",
                    yukino::db::ty::DatabaseType::UnsignedSmallInteger,
                )]))),
        }
    }
}
impl Entity for Basic {
    type View = BasicView;
}
impl Value for Basic {
    fn converter() -> ConverterRef<Self>
    where
        Self: Sized,
    {
        BasicConverter::instance()
    }
}
#[derive(Clone)]
pub struct BasicConverter;
unsafe impl Sync for BasicConverter {}
static BASIC_CONVERTER: BasicConverter = BasicConverter;
impl Converter for BasicConverter {
    type Output = Basic;
    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &BASIC_CONVERTER
    }
    fn param_count(&self) -> usize {
        12usize
    }
    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> ConvertResult<Self::Output>> {
        Box::new(|v| {
            Ok(Basic {
                character: (*<char>::converter().deserializer())(&v[0usize..1usize])?,
                double: (*<f64>::converter().deserializer())(&v[1usize..2usize])?,
                float: (*<f32>::converter().deserializer())(&v[2usize..3usize])?,
                id: (*<u32>::converter().deserializer())(&v[3usize..4usize])?,
                int: (*<i32>::converter().deserializer())(&v[4usize..5usize])?,
                long: (*<i64>::converter().deserializer())(&v[5usize..6usize])?,
                optional: (*<Option<u32>>::converter().deserializer())(&v[6usize..7usize])?,
                short: (*<i16>::converter().deserializer())(&v[7usize..8usize])?,
                string: (*<String>::converter().deserializer())(&v[8usize..9usize])?,
                u_int: (*<u32>::converter().deserializer())(&v[9usize..10usize])?,
                u_long: (*<u64>::converter().deserializer())(&v[10usize..11usize])?,
                u_short: (*<u16>::converter().deserializer())(&v[11usize..12usize])?,
            })
        })
    }
    fn serialize(&self, value: &Self::Output) -> ConvertResult<Vec<DatabaseValue>> {
        Ok(vec![
            <char>::converter().serialize(&value.character)?,
            <f64>::converter().serialize(&value.double)?,
            <f32>::converter().serialize(&value.float)?,
            <u32>::converter().serialize(&value.id)?,
            <i32>::converter().serialize(&value.int)?,
            <i64>::converter().serialize(&value.long)?,
            <Option<u32>>::converter().serialize(&value.optional)?,
            <i16>::converter().serialize(&value.short)?,
            <String>::converter().serialize(&value.string)?,
            <u32>::converter().serialize(&value.u_int)?,
            <u64>::converter().serialize(&value.u_long)?,
            <u16>::converter().serialize(&value.u_short)?,
        ]
        .into_iter()
        .flatten()
        .collect())
    }
}
pub mod basic {
    use lazy_static::lazy_static;
    use yukino::interface::def::FieldDefinition;
    use yukino::interface::FieldMarker;
    #[allow(non_camel_case_types)]
    pub struct character();
    lazy_static! {
        static ref CHARACTER_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "character".to_string(),
                "char".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "character".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "character".to_string(),
                        yukino::db::ty::DatabaseType::Character,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["character".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for character {
        type ValueType = char;
        fn field_name() -> &'static str {
            "character"
        }
        fn definition() -> &'static FieldDefinition {
            &*CHARACTER_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct double();
    lazy_static! {
        static ref DOUBLE_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "double".to_string(),
                "f64".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "double".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "double".to_string(),
                        yukino::db::ty::DatabaseType::Double,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["double".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for double {
        type ValueType = f64;
        fn field_name() -> &'static str {
            "double"
        }
        fn definition() -> &'static FieldDefinition {
            &*DOUBLE_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct float();
    lazy_static! {
        static ref FLOAT_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "float".to_string(),
            "f32".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "float".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "float".to_string(),
                    yukino::db::ty::DatabaseType::Float,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["float".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for float {
        type ValueType = f32;
        fn field_name() -> &'static str {
            "float"
        }
        fn definition() -> &'static FieldDefinition {
            &*FLOAT_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct id();
    lazy_static! {
        static ref ID_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "id".to_string(),
            "u32".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "id".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "id".to_string(),
                    yukino::db::ty::DatabaseType::UnsignedInteger,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["id".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for id {
        type ValueType = u32;
        fn field_name() -> &'static str {
            "id"
        }
        fn definition() -> &'static FieldDefinition {
            &*ID_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct int();
    lazy_static! {
        static ref INT_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "int".to_string(),
            "i32".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "int".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "int".to_string(),
                    yukino::db::ty::DatabaseType::Integer,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["int".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for int {
        type ValueType = i32;
        fn field_name() -> &'static str {
            "int"
        }
        fn definition() -> &'static FieldDefinition {
            &*INT_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct long();
    lazy_static! {
        static ref LONG_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "long".to_string(),
            "i64".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "long".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "long".to_string(),
                    yukino::db::ty::DatabaseType::BigInteger,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["long".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for long {
        type ValueType = i64;
        fn field_name() -> &'static str {
            "long"
        }
        fn definition() -> &'static FieldDefinition {
            &*LONG_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct optional();
    lazy_static! {
        static ref OPTIONAL_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "optional".to_string(),
                "Option < u32 >".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "optional".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "optional".to_string(),
                        yukino::db::ty::DatabaseType::UnsignedInteger,
                        true,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["optional".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for optional {
        type ValueType = Option<u32>;
        fn field_name() -> &'static str {
            "optional"
        }
        fn definition() -> &'static FieldDefinition {
            &*OPTIONAL_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct short();
    lazy_static! {
        static ref SHORT_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "short".to_string(),
            "i16".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "short".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "short".to_string(),
                    yukino::db::ty::DatabaseType::SmallInteger,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["short".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for short {
        type ValueType = i16;
        fn field_name() -> &'static str {
            "short"
        }
        fn definition() -> &'static FieldDefinition {
            &*SHORT_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct string();
    lazy_static! {
        static ref STRING_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "string".to_string(),
                "String".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "string".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "string".to_string(),
                        yukino::db::ty::DatabaseType::String,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["string".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for string {
        type ValueType = String;
        fn field_name() -> &'static str {
            "string"
        }
        fn definition() -> &'static FieldDefinition {
            &*STRING_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_int();
    lazy_static! {
        static ref U_INT_DEFINITION: FieldDefinition = yukino::interface::def::FieldDefinition::new(
            "u_int".to_string(),
            "u32".to_string(),
            false,
            yukino::interface::def::DefinitionType::Normal,
            vec![(
                "u_int".to_string(),
                yukino::interface::def::ColumnDefinition::new(
                    "u_int".to_string(),
                    yukino::db::ty::DatabaseType::UnsignedInteger,
                    false,
                    false
                )
            )]
            .into_iter()
            .collect(),
            vec!["u_int".to_string()],
            None,
            vec![]
        );
    }
    impl FieldMarker for u_int {
        type ValueType = u32;
        fn field_name() -> &'static str {
            "u_int"
        }
        fn definition() -> &'static FieldDefinition {
            &*U_INT_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_long();
    lazy_static! {
        static ref U_LONG_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "u_long".to_string(),
                "u64".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "u_long".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "u_long".to_string(),
                        yukino::db::ty::DatabaseType::UnsignedBigInteger,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["u_long".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for u_long {
        type ValueType = u64;
        fn field_name() -> &'static str {
            "u_long"
        }
        fn definition() -> &'static FieldDefinition {
            &*U_LONG_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_short();
    lazy_static! {
        static ref U_SHORT_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "u_short".to_string(),
                "u16".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "u_short".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "u_short".to_string(),
                        yukino::db::ty::DatabaseType::UnsignedSmallInteger,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["u_short".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for u_short {
        type ValueType = u16;
        fn field_name() -> &'static str {
            "u_short"
        }
        fn definition() -> &'static FieldDefinition {
            &*U_SHORT_DEFINITION
        }
    }
}
