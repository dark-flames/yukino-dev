use lazy_static::lazy_static;
use yukino::converter::Converter;
use yukino::db::ty::DatabaseValue;
use yukino::db::ty::ValuePack;
use yukino::err::RuntimeResult;
use yukino::expr::Expr;
use yukino::expr::Value;
use yukino::expr::{Computation, ComputationNode, Node, QueryResultNode};
use yukino::interface::Entity;
use yukino::interface::EntityView;
use yukino::interface::FieldMarker;
use yukino::query::SelectedItem;
use yukino::view::View;
#[derive(Clone)]
pub struct Numeric {
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
#[derive(Clone)]
pub struct NumericView {
    pub character: Expr<char>,
    pub double: Expr<f64>,
    pub float: Expr<f32>,
    pub id: Expr<u32>,
    pub int: Expr<i32>,
    pub long: Expr<i64>,
    pub optional: Expr<Option<u32>>,
    pub short: Expr<i16>,
    pub string: Expr<String>,
    pub u_int: Expr<u32>,
    pub u_long: Expr<u64>,
    pub u_short: Expr<u16>,
}
unsafe impl Sync for NumericView {}
static NUMERIC_CONVERTER: NumericConverter = NumericConverter();
impl Node for NumericView {
    fn collect_selected_items(&self) -> Vec<SelectedItem> {
        let mut result = vec![];
        result.extend(self.character.collect_selected_items());
        result.extend(self.double.collect_selected_items());
        result.extend(self.float.collect_selected_items());
        result.extend(self.id.collect_selected_items());
        result.extend(self.int.collect_selected_items());
        result.extend(self.long.collect_selected_items());
        result.extend(self.optional.collect_selected_items());
        result.extend(self.short.collect_selected_items());
        result.extend(self.string.collect_selected_items());
        result.extend(self.u_int.collect_selected_items());
        result.extend(self.u_long.collect_selected_items());
        result.extend(self.u_short.collect_selected_items());
        result
    }
    fn converter(&self) -> &'static dyn Converter<Output = Self::Output> {
        &NUMERIC_CONVERTER
    }
}
impl Computation for NumericView {
    type Output = Numeric;
    fn eval(&self, v: &ValuePack) -> RuntimeResult<Self::Output> {
        Ok(Numeric {
            character: self.character.eval(v)?,
            double: self.double.eval(v)?,
            float: self.float.eval(v)?,
            id: self.id.eval(v)?,
            int: self.int.eval(v)?,
            long: self.long.eval(v)?,
            optional: self.optional.eval(v)?,
            short: self.short.eval(v)?,
            string: self.string.eval(v)?,
            u_int: self.u_int.eval(v)?,
            u_long: self.u_long.eval(v)?,
            u_short: self.u_short.eval(v)?,
        })
    }
}
lazy_static! {
    static ref NUMERIC_VIEW: NumericView = NumericView {
        character: Expr::QueryResult(QueryResultNode {
            converter: numeric::character::converter(),
            aliases: vec!["character".to_string()]
        }),
        double: Expr::QueryResult(QueryResultNode {
            converter: numeric::double::converter(),
            aliases: vec!["double".to_string()]
        }),
        float: Expr::QueryResult(QueryResultNode {
            converter: numeric::float::converter(),
            aliases: vec!["float".to_string()]
        }),
        id: Expr::QueryResult(QueryResultNode {
            converter: numeric::id::converter(),
            aliases: vec!["id".to_string()]
        }),
        int: Expr::QueryResult(QueryResultNode {
            converter: numeric::int::converter(),
            aliases: vec!["int".to_string()]
        }),
        long: Expr::QueryResult(QueryResultNode {
            converter: numeric::long::converter(),
            aliases: vec!["long".to_string()]
        }),
        optional: Expr::QueryResult(QueryResultNode {
            converter: numeric::optional::converter(),
            aliases: vec!["optional".to_string()]
        }),
        short: Expr::QueryResult(QueryResultNode {
            converter: numeric::short::converter(),
            aliases: vec!["short".to_string()]
        }),
        string: Expr::QueryResult(QueryResultNode {
            converter: numeric::string::converter(),
            aliases: vec!["string".to_string()]
        }),
        u_int: Expr::QueryResult(QueryResultNode {
            converter: numeric::u_int::converter(),
            aliases: vec!["u_int".to_string()]
        }),
        u_long: Expr::QueryResult(QueryResultNode {
            converter: numeric::u_long::converter(),
            aliases: vec!["u_long".to_string()]
        }),
        u_short: Expr::QueryResult(QueryResultNode {
            converter: numeric::u_short::converter(),
            aliases: vec!["u_short".to_string()]
        })
    };
}
impl View for NumericView {
    type Output = Numeric;
    fn expr(&self) -> Expr<Self::Output> {
        Expr::Computation(Box::new(NumericView::pure()))
    }
}
impl ComputationNode for NumericView {
    fn box_clone(&self) -> Box<dyn ComputationNode<Output = Self::Output>> {
        Box::new(self.clone())
    }
}
impl EntityView for NumericView {
    type Entity = Numeric;
    fn static_ref() -> &'static Self
    where
        Self: Sized,
    {
        &*NUMERIC_VIEW
    }
}
impl Entity for Numeric {
    type View = NumericView;
}
impl Value for Numeric {}
#[derive(Clone)]
pub struct NumericConverter();
unsafe impl Sync for NumericConverter {}
impl Converter for NumericConverter {
    type Output = Numeric;
    fn deserializer(&self) -> Box<dyn Fn(&[&DatabaseValue]) -> RuntimeResult<Self::Output>> {
        Box::new(|v| {
            Ok(Numeric {
                character: (*numeric::character::converter().deserializer())(&v[0usize..1usize])?,
                double: (*numeric::double::converter().deserializer())(&v[1usize..2usize])?,
                float: (*numeric::float::converter().deserializer())(&v[2usize..3usize])?,
                id: (*numeric::id::converter().deserializer())(&v[3usize..4usize])?,
                int: (*numeric::int::converter().deserializer())(&v[4usize..5usize])?,
                long: (*numeric::long::converter().deserializer())(&v[5usize..6usize])?,
                optional: (*numeric::optional::converter().deserializer())(&v[6usize..7usize])?,
                short: (*numeric::short::converter().deserializer())(&v[7usize..8usize])?,
                string: (*numeric::string::converter().deserializer())(&v[8usize..9usize])?,
                u_int: (*numeric::u_int::converter().deserializer())(&v[9usize..10usize])?,
                u_long: (*numeric::u_long::converter().deserializer())(&v[10usize..11usize])?,
                u_short: (*numeric::u_short::converter().deserializer())(&v[11usize..12usize])?,
            })
        })
    }
    fn serialize(&self, value: &Self::Output) -> RuntimeResult<Vec<DatabaseValue>> {
        Ok(vec![
            numeric::character::converter().serialize(&value.character)?,
            numeric::double::converter().serialize(&value.double)?,
            numeric::float::converter().serialize(&value.float)?,
            numeric::id::converter().serialize(&value.id)?,
            numeric::int::converter().serialize(&value.int)?,
            numeric::long::converter().serialize(&value.long)?,
            numeric::optional::converter().serialize(&value.optional)?,
            numeric::short::converter().serialize(&value.short)?,
            numeric::string::converter().serialize(&value.string)?,
            numeric::u_int::converter().serialize(&value.u_int)?,
            numeric::u_long::converter().serialize(&value.u_long)?,
            numeric::u_short::converter().serialize(&value.u_short)?,
        ]
        .into_iter()
        .flatten()
        .collect())
    }
}
impl Value for NumericConverter {}
pub mod numeric {
    use super::NumericView;
    use lazy_static::lazy_static;
    use yukino::converter::Converter;
    use yukino::expr::Expr;
    use yukino::interface::def::FieldDefinition;
    use yukino::interface::EntityView;
    use yukino::interface::FieldMarker;
    #[allow(non_camel_case_types)]
    pub struct character();
    lazy_static! {
        static ref CHARACTER_CONVERTER: yukino::converter::basic::CharConverter =
            yukino::converter::basic::CharConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*CHARACTER_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*CHARACTER_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().character
        }
    }
    #[allow(non_camel_case_types)]
    pub struct double();
    lazy_static! {
        static ref DOUBLE_CONVERTER: yukino::converter::basic::DoubleConverter =
            yukino::converter::basic::DoubleConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*DOUBLE_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*DOUBLE_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().double
        }
    }
    #[allow(non_camel_case_types)]
    pub struct float();
    lazy_static! {
        static ref FLOAT_CONVERTER: yukino::converter::basic::FloatConverter =
            yukino::converter::basic::FloatConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*FLOAT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*FLOAT_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().float
        }
    }
    #[allow(non_camel_case_types)]
    pub struct id();
    lazy_static! {
        static ref ID_CONVERTER: yukino::converter::basic::UnsignedIntConverter =
            yukino::converter::basic::UnsignedIntConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*ID_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*ID_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().id
        }
    }
    #[allow(non_camel_case_types)]
    pub struct int();
    lazy_static! {
        static ref INT_CONVERTER: yukino::converter::basic::IntConverter =
            yukino::converter::basic::IntConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*INT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*INT_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().int
        }
    }
    #[allow(non_camel_case_types)]
    pub struct long();
    lazy_static! {
        static ref LONG_CONVERTER: yukino::converter::basic::LongConverter =
            yukino::converter::basic::LongConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*LONG_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().long
        }
    }
    #[allow(non_camel_case_types)]
    pub struct optional();
    lazy_static! {
        static ref OPTIONAL_CONVERTER: yukino::converter::basic::OptionalUnsignedIntConverter =
            yukino::converter::basic::OptionalUnsignedIntConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*OPTIONAL_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*OPTIONAL_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().optional
        }
    }
    #[allow(non_camel_case_types)]
    pub struct short();
    lazy_static! {
        static ref SHORT_CONVERTER: yukino::converter::basic::ShortConverter =
            yukino::converter::basic::ShortConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*SHORT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*SHORT_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().short
        }
    }
    #[allow(non_camel_case_types)]
    pub struct string();
    lazy_static! {
        static ref STRING_CONVERTER: yukino::converter::basic::StringConverter =
            yukino::converter::basic::StringConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*STRING_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*STRING_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().string
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_int();
    lazy_static! {
        static ref U_INT_CONVERTER: yukino::converter::basic::UnsignedIntConverter =
            yukino::converter::basic::UnsignedIntConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*U_INT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_INT_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().u_int
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_long();
    lazy_static! {
        static ref U_LONG_CONVERTER: yukino::converter::basic::UnsignedLongConverter =
            yukino::converter::basic::UnsignedLongConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*U_LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_LONG_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().u_long
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_short();
    lazy_static! {
        static ref U_SHORT_CONVERTER: yukino::converter::basic::UnsignedShortConverter =
            yukino::converter::basic::UnsignedShortConverter();
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
        fn converter() -> &'static dyn Converter<Output = Self::ValueType> {
            &*U_SHORT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_SHORT_DEFINITION
        }
        fn view() -> &'static Expr<Self::ValueType> {
            &NumericView::static_ref().u_short
        }
    }
}
