use yukino::interface::Entity;
use yukino::interface::EntityView;
use yukino::interface::{FieldMarker, FieldView};
use yukino::query::computation::Computation;
use yukino::query::optimizer::{QueryOptimizer, SelectAppendOptimizer};
use yukino::view::*;
#[derive(Clone)]
pub struct Numeric {
    pub u_short: u16,
    pub u_int: u32,
    pub short: i16,
    pub id: u32,
    pub u_long: u64,
    pub int: i32,
    pub long: i64,
    pub string: String,
    pub double: f64,
    pub float: f32,
    pub character: char,
}
pub struct NumericView {
    pub u_short: UnsignedShortFieldView,
    pub u_int: UnsignedIntFieldView,
    pub short: ShortFieldView,
    pub id: UnsignedIntFieldView,
    pub u_long: UnsignedLongFieldView,
    pub int: IntFieldView,
    pub long: LongFieldView,
    pub string: StringFieldView,
    pub double: DoubleFieldView,
    pub float: FloatFieldView,
    pub character: CharFieldView,
}
impl View for NumericView {
    type Output = Numeric;
    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        Computation::create(Box::new(|v| {
            Ok(Numeric {
                u_short: { (*numeric::u_short::data_converter().field_value_converter())(v)? },
                u_int: { (*numeric::u_int::data_converter().field_value_converter())(v)? },
                short: { (*numeric::short::data_converter().field_value_converter())(v)? },
                id: { (*numeric::id::data_converter().field_value_converter())(v)? },
                u_long: { (*numeric::u_long::data_converter().field_value_converter())(v)? },
                int: { (*numeric::int::data_converter().field_value_converter())(v)? },
                long: { (*numeric::long::data_converter().field_value_converter())(v)? },
                string: { (*numeric::string::data_converter().field_value_converter())(v)? },
                double: { (*numeric::double::data_converter().field_value_converter())(v)? },
                float: { (*numeric::float::data_converter().field_value_converter())(v)? },
                character: { (*numeric::character::data_converter().field_value_converter())(v)? },
            })
        }))
    }
    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        let mut optimizer: SelectAppendOptimizer = Default::default();
        optimizer
            .append::<numeric::u_short>()
            .append::<numeric::u_int>()
            .append::<numeric::short>()
            .append::<numeric::id>()
            .append::<numeric::u_long>()
            .append::<numeric::int>()
            .append::<numeric::long>()
            .append::<numeric::string>()
            .append::<numeric::double>()
            .append::<numeric::float>()
            .append::<numeric::character>();
        Box::new(optimizer)
    }
}
impl EntityView for NumericView {
    type Entity = Numeric;
    fn pure() -> Self
    where
        Self: Sized,
    {
        NumericView {
            u_short: UnsignedShortFieldView::create(numeric::u_short::data_converter()),
            u_int: UnsignedIntFieldView::create(numeric::u_int::data_converter()),
            short: ShortFieldView::create(numeric::short::data_converter()),
            id: UnsignedIntFieldView::create(numeric::id::data_converter()),
            u_long: UnsignedLongFieldView::create(numeric::u_long::data_converter()),
            int: IntFieldView::create(numeric::int::data_converter()),
            long: LongFieldView::create(numeric::long::data_converter()),
            string: StringFieldView::create(numeric::string::data_converter()),
            double: DoubleFieldView::create(numeric::double::data_converter()),
            float: FloatFieldView::create(numeric::float::data_converter()),
            character: CharFieldView::create(numeric::character::data_converter()),
        }
    }
}
impl Entity for Numeric {
    type View = NumericView;
}
pub mod numeric {
    use lazy_static::lazy_static;
    use yukino::interface::converter::DataConverter;
    use yukino::interface::def::FieldDefinition;
    use yukino::interface::FieldMarker;

    #[allow(non_camel_case_types)]
    pub struct u_short();
    lazy_static! {
        static ref U_SHORT_CONVERTER: yukino::converter::basic::UnsignedShortDataConverter =
            yukino::converter::basic::UnsignedShortDataConverter::new("u_short".to_string());
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
        type Type = u16;
        fn field_name() -> &'static str {
            "u_short"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*U_SHORT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_SHORT_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct u_int();
    lazy_static! {
        static ref U_INT_CONVERTER: yukino::converter::basic::UnsignedIntDataConverter =
            yukino::converter::basic::UnsignedIntDataConverter::new("u_int".to_string());
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
        type Type = u32;
        fn field_name() -> &'static str {
            "u_int"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*U_INT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_INT_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct short();
    lazy_static! {
        static ref SHORT_CONVERTER: yukino::converter::basic::ShortDataConverter =
            yukino::converter::basic::ShortDataConverter::new("short".to_string());
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
        type Type = i16;
        fn field_name() -> &'static str {
            "short"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*SHORT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*SHORT_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct id();
    lazy_static! {
        static ref ID_CONVERTER: yukino::converter::basic::UnsignedIntDataConverter =
            yukino::converter::basic::UnsignedIntDataConverter::new("id".to_string());
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
        type Type = u32;
        fn field_name() -> &'static str {
            "id"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*ID_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*ID_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct u_long();
    lazy_static! {
        static ref U_LONG_CONVERTER: yukino::converter::basic::UnsignedLongDataConverter =
            yukino::converter::basic::UnsignedLongDataConverter::new("u_long".to_string());
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
        type Type = u64;
        fn field_name() -> &'static str {
            "u_long"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*U_LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_LONG_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct int();
    lazy_static! {
        static ref INT_CONVERTER: yukino::converter::basic::IntDataConverter =
            yukino::converter::basic::IntDataConverter::new("int".to_string());
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
        type Type = i32;
        fn field_name() -> &'static str {
            "int"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*INT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*INT_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct long();
    lazy_static! {
        static ref LONG_CONVERTER: yukino::converter::basic::LongDataConverter =
            yukino::converter::basic::LongDataConverter::new("long".to_string());
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
        type Type = i64;
        fn field_name() -> &'static str {
            "long"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*LONG_DEFINITION
        }
    }
    #[allow(non_camel_case_types)]
    pub struct string();
    lazy_static! {
        static ref STRING_CONVERTER: yukino::converter::basic::StringDataConverter =
            yukino::converter::basic::StringDataConverter::new("string".to_string());
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
        type Type = String;
        fn field_name() -> &'static str {
            "string"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*STRING_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*STRING_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct double();
    lazy_static! {
        static ref DOUBLE_CONVERTER: yukino::converter::basic::DoubleDataConverter =
            yukino::converter::basic::DoubleDataConverter::new("double".to_string());
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
        type Type = f64;
        fn field_name() -> &'static str {
            "double"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*DOUBLE_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*DOUBLE_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct float();
    lazy_static! {
        static ref FLOAT_CONVERTER: yukino::converter::basic::FloatDataConverter =
            yukino::converter::basic::FloatDataConverter::new("float".to_string());
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
        type Type = f32;
        fn field_name() -> &'static str {
            "float"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*FLOAT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*FLOAT_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct character();
    lazy_static! {
        static ref CHARACTER_CONVERTER: yukino::converter::basic::CharDataConverter =
            yukino::converter::basic::CharDataConverter::new("character".to_string());
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
        type Type = char;
        fn field_name() -> &'static str {
            "character"
        }
        fn data_converter() -> &'static dyn DataConverter<FieldType=Self::Type> {
            &*CHARACTER_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*CHARACTER_DEFINITION
        }
    }
}
