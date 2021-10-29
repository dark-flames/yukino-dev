use yukino::interface::Entity;
use yukino::interface::EntityView;
use yukino::interface::{FieldMarker, FieldView};
use yukino::query::computation::Computation;
use yukino::query::optimizer::{QueryOptimizer, SelectAppendOptimizer};
use yukino::view::*;
#[derive(Clone)]
pub struct Numeric {
    pub id: u32,
    pub u_short: u16,
    pub u_int: u32,
    pub u_long: u64,
    pub float: f32,
    pub optional: Option<u32>,
    pub int: i32,
    pub long: i64,
    pub short: i16,
    pub double: f64,
    pub string: String,
    pub character: char,
}
pub struct NumericView {
    pub id: UnsignedIntFieldView,
    pub u_short: UnsignedShortFieldView,
    pub u_int: UnsignedIntFieldView,
    pub u_long: UnsignedLongFieldView,
    pub float: FloatFieldView,
    pub optional: OptionalFieldWrapper<UnsignedIntFieldView>,
    pub int: IntFieldView,
    pub long: LongFieldView,
    pub short: ShortFieldView,
    pub double: DoubleFieldView,
    pub string: StringFieldView,
    pub character: CharFieldView,
}
impl View for NumericView {
    type Output = Numeric;
    fn computation<'f>(&self) -> Computation<'f, Self::Output> {
        let id_computation = self.id.computation();
        let u_short_computation = self.u_short.computation();
        let u_int_computation = self.u_int.computation();
        let u_long_computation = self.u_long.computation();
        let float_computation = self.float.computation();
        let optional_computation = self.optional.computation();
        let int_computation = self.int.computation();
        let long_computation = self.long.computation();
        let short_computation = self.short.computation();
        let double_computation = self.double.computation();
        let string_computation = self.string.computation();
        let character_computation = self.character.computation();
        Computation::create(Box::new(move |v| {
            Ok(Numeric {
                id: id_computation.eval(v)?,
                u_short: u_short_computation.eval(v)?,
                u_int: u_int_computation.eval(v)?,
                u_long: u_long_computation.eval(v)?,
                float: float_computation.eval(v)?,
                optional: optional_computation.eval(v)?,
                int: int_computation.eval(v)?,
                long: long_computation.eval(v)?,
                short: short_computation.eval(v)?,
                double: double_computation.eval(v)?,
                string: string_computation.eval(v)?,
                character: character_computation.eval(v)?,
            })
        }))
    }
    fn optimizer(&self) -> Box<dyn QueryOptimizer> {
        let mut optimizer: SelectAppendOptimizer = Default::default();
        optimizer
            .append::<numeric::id>()
            .append::<numeric::u_short>()
            .append::<numeric::u_int>()
            .append::<numeric::u_long>()
            .append::<numeric::float>()
            .append::<numeric::optional>()
            .append::<numeric::int>()
            .append::<numeric::long>()
            .append::<numeric::short>()
            .append::<numeric::double>()
            .append::<numeric::string>()
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
            id: UnsignedIntFieldView::create(numeric::id::data_converter()),
            u_short: UnsignedShortFieldView::create(numeric::u_short::data_converter()),
            u_int: UnsignedIntFieldView::create(numeric::u_int::data_converter()),
            u_long: UnsignedLongFieldView::create(numeric::u_long::data_converter()),
            float: FloatFieldView::create(numeric::float::data_converter()),
            optional: OptionalFieldWrapper::<UnsignedIntFieldView>::create(
                numeric::optional::data_converter(),
            ),
            int: IntFieldView::create(numeric::int::data_converter()),
            long: LongFieldView::create(numeric::long::data_converter()),
            short: ShortFieldView::create(numeric::short::data_converter()),
            double: DoubleFieldView::create(numeric::double::data_converter()),
            string: StringFieldView::create(numeric::string::data_converter()),
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
        type ValueType = u32;
        fn field_name() -> &'static str {
            "id"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*ID_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*ID_DEFINITION
        }
    }

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
        type ValueType = u16;
        fn field_name() -> &'static str {
            "u_short"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
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
        type ValueType = u32;
        fn field_name() -> &'static str {
            "u_int"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*U_INT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_INT_DEFINITION
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
        type ValueType = u64;
        fn field_name() -> &'static str {
            "u_long"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*U_LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*U_LONG_DEFINITION
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
        type ValueType = f32;
        fn field_name() -> &'static str {
            "float"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*FLOAT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*FLOAT_DEFINITION
        }
    }

    #[allow(non_camel_case_types)]
    pub struct optional();
    lazy_static! {
        static ref OPTIONAL_CONVERTER: yukino::converter::basic::UnsignedIntDataConverter =
            yukino::converter::basic::UnsignedIntDataConverter::new("optional".to_string());
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
        type ValueType = u32;
        fn field_name() -> &'static str {
            "optional"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*OPTIONAL_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*OPTIONAL_DEFINITION
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
        type ValueType = i32;
        fn field_name() -> &'static str {
            "int"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
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
        type ValueType = i64;
        fn field_name() -> &'static str {
            "long"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*LONG_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*LONG_DEFINITION
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
        type ValueType = i16;
        fn field_name() -> &'static str {
            "short"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*SHORT_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*SHORT_DEFINITION
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
        type ValueType = f64;
        fn field_name() -> &'static str {
            "double"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*DOUBLE_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*DOUBLE_DEFINITION
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
        type ValueType = String;
        fn field_name() -> &'static str {
            "string"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*STRING_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*STRING_DEFINITION
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
        type ValueType = char;
        fn field_name() -> &'static str {
            "character"
        }
        fn data_converter() -> &'static dyn DataConverter<Output=Self::ValueType> {
            &*CHARACTER_CONVERTER
        }
        fn definition() -> &'static FieldDefinition {
            &*CHARACTER_DEFINITION
        }
    }
}
