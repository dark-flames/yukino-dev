use yukino::{EntityDefinition, YukinoEntity};
use yukino::converter::{Converter, ConverterInstance, ConvertResult, Deserializer};
use yukino::converter::ConverterRef;
use yukino::DefinitionManager;
use yukino::err::{RuntimeResult, YukinoError};
use yukino::generic_array::{arr, GenericArray};
use yukino::generic_array::sequence::{Concat, Split};
use yukino::generic_array::typenum;
use yukino::lazy_static::lazy_static;
use yukino::query_builder::{Alias, DatabaseValue, Expr};
use yukino::query_result::{ExprMutVisitor, ExprNode, ExprVisitor};
use yukino::query_result::QueryResultFilter;
use yukino::view::{
    EntityView, EntityViewTag, ExprView, ExprViewBox, ExprViewBoxWithTag, SingleExprView, TagList1,
};
use yukino::view::{EntityWithView, Value, ValueCountOf};

#[derive(Clone, Debug)]
pub struct Basic {
    pub boolean: bool,
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
pub struct BasicView {
    pub boolean: ExprViewBox<bool>,
    pub character: ExprViewBox<char>,
    pub double: ExprViewBox<f64>,
    pub float: ExprViewBox<f32>,
    pub id: ExprViewBox<u32>,
    pub int: ExprViewBox<i32>,
    pub long: ExprViewBox<i64>,
    pub optional: ExprViewBox<Option<u32>>,
    pub short: ExprViewBox<i16>,
    pub string: ExprViewBox<String>,
    pub u_int: ExprViewBox<u32>,
    pub u_long: ExprViewBox<u64>,
    pub u_short: ExprViewBox<u16>,
}
impl ExprNode for BasicView {
    fn apply(&self, visitor: &mut dyn ExprVisitor) {
        self.boolean.apply(visitor);
        self.character.apply(visitor);
        self.double.apply(visitor);
        self.float.apply(visitor);
        self.id.apply(visitor);
        self.int.apply(visitor);
        self.long.apply(visitor);
        self.optional.apply(visitor);
        self.short.apply(visitor);
        self.string.apply(visitor);
        self.u_int.apply(visitor);
        self.u_long.apply(visitor);
        self.u_short.apply(visitor);
    }
    fn apply_mut(&mut self, visitor: &mut dyn ExprMutVisitor) {
        self.boolean.apply_mut(visitor);
        self.character.apply_mut(visitor);
        self.double.apply_mut(visitor);
        self.float.apply_mut(visitor);
        self.id.apply_mut(visitor);
        self.int.apply_mut(visitor);
        self.long.apply_mut(visitor);
        self.optional.apply_mut(visitor);
        self.short.apply_mut(visitor);
        self.string.apply_mut(visitor);
        self.u_int.apply_mut(visitor);
        self.u_long.apply_mut(visitor);
        self.u_short.apply_mut(visitor);
    }
}
impl ExprView<Basic> for BasicView {
    type Tags = TagList1<EntityViewTag>;
    fn from_exprs(
        exprs: GenericArray<Expr, ValueCountOf<Basic>>,
    ) -> ExprViewBoxWithTag<Basic, Self::Tags>
    where
        Self: Sized,
    {
        let rest = exprs;
        let (boolean, rest) = Split::<_, typenum::U1>::split(rest);
        let (character, rest) = Split::<_, typenum::U1>::split(rest);
        let (double, rest) = Split::<_, typenum::U1>::split(rest);
        let (float, rest) = Split::<_, typenum::U1>::split(rest);
        let (id, rest) = Split::<_, typenum::U1>::split(rest);
        let (int, rest) = Split::<_, typenum::U1>::split(rest);
        let (long, rest) = Split::<_, typenum::U1>::split(rest);
        let (optional, rest) = Split::<_, typenum::U1>::split(rest);
        let (short, rest) = Split::<_, typenum::U1>::split(rest);
        let (string, rest) = Split::<_, typenum::U1>::split(rest);
        let (u_int, rest) = Split::<_, typenum::U1>::split(rest);
        let (u_long, rest) = Split::<_, typenum::U1>::split(rest);
        let (u_short, _) = Split::<_, typenum::U1>::split(rest);
        Box::new(BasicView {
            boolean: SingleExprView::<bool>::from_exprs(boolean),
            character: SingleExprView::<char>::from_exprs(character),
            double: SingleExprView::<f64>::from_exprs(double),
            float: SingleExprView::<f32>::from_exprs(float),
            id: SingleExprView::<u32>::from_exprs(id),
            int: SingleExprView::<i32>::from_exprs(int),
            long: SingleExprView::<i64>::from_exprs(long),
            optional: SingleExprView::<Option<u32>>::from_exprs(optional),
            short: SingleExprView::<i16>::from_exprs(short),
            string: SingleExprView::<String>::from_exprs(string),
            u_int: SingleExprView::<u32>::from_exprs(u_int),
            u_long: SingleExprView::<u64>::from_exprs(u_long),
            u_short: SingleExprView::<u16>::from_exprs(u_short),
        })
    }
    fn clone_expr_view(&self) -> Self
    where
        Self: Sized,
    {
        Clone::clone(self)
    }
    fn expr_clone(&self) -> ExprViewBoxWithTag<Basic, Self::Tags>
    where
        Self: Sized,
    {
        Box::new(BasicView {
            boolean: self.boolean.clone(),
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
        })
    }
    fn collect_expr(&self) -> GenericArray<Expr, ValueCountOf<Basic>> {
        let boolean = self.boolean.collect_expr();
        let character = self.character.collect_expr();
        let double = self.double.collect_expr();
        let float = self.float.collect_expr();
        let id = self.id.collect_expr();
        let int = self.int.collect_expr();
        let long = self.long.collect_expr();
        let optional = self.optional.collect_expr();
        let short = self.short.collect_expr();
        let string = self.string.collect_expr();
        let u_int = self.u_int.collect_expr();
        let u_long = self.u_long.collect_expr();
        let u_short = self.u_short.collect_expr();
        Concat::concat(
            Concat::concat(
                Concat::concat(
                    Concat::concat(
                        Concat::concat(
                            Concat::concat(
                                Concat::concat(
                                    Concat::concat(
                                        Concat::concat(
                                            Concat::concat(
                                                Concat::concat(
                                                    Concat::concat(
                                                        Concat::concat(arr ! [Expr ;], boolean),
                                                        character,
                                                    ),
                                                    double,
                                                ),
                                                float,
                                            ),
                                            id,
                                        ),
                                        int,
                                    ),
                                    long,
                                ),
                                optional,
                            ),
                            short,
                        ),
                        string,
                    ),
                    u_int,
                ),
                u_long,
            ),
            u_short,
        )
    }
    fn eval(&self, v: &GenericArray<DatabaseValue, ValueCountOf<Basic>>) -> RuntimeResult<Basic> {
        (*Basic::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
}
impl EntityView for BasicView {
    type Entity = Basic;
    fn pure(alias: &Alias) -> Self
    where
        Self: Sized,
    {
        BasicView {
            boolean: SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("boolean")],
            ),
            character: SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("character")],
            ),
            double: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("double")]),
            float: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("float")]),
            id: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("id")]),
            int: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("int")]),
            long: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("long")]),
            optional: SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("optional")],
            ),
            short: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("short")]),
            string: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("string")]),
            u_int: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("u_int")]),
            u_long: SingleExprView::from_exprs(arr ! [Expr ; alias . create_ident_expr ("u_long")]),
            u_short: SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("u_short")],
            ),
        }
    }
}
lazy_static! {
    static ref BASIC_DEFINITION: EntityDefinition = yukino::EntityDefinition::new(
        0usize,
        "basic".to_string(),
        yukino::DefinitionType::Normal,
        vec![
            (
                "character".to_string(),
                yukino::FieldDefinition::new(
                    "character".to_string(),
                    "char".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "character".to_string(),
                        yukino::ColumnDefinition::new(
                            "character".to_string(),
                            yukino::DatabaseType::Character,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["character".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "u_short".to_string(),
                yukino::FieldDefinition::new(
                    "u_short".to_string(),
                    "u16".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "u_short".to_string(),
                        yukino::ColumnDefinition::new(
                            "u_short".to_string(),
                            yukino::DatabaseType::UnsignedSmallInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["u_short".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "boolean".to_string(),
                yukino::FieldDefinition::new(
                    "boolean".to_string(),
                    "bool".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "boolean".to_string(),
                        yukino::ColumnDefinition::new(
                            "boolean".to_string(),
                            yukino::DatabaseType::Bool,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["boolean".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "u_long".to_string(),
                yukino::FieldDefinition::new(
                    "u_long".to_string(),
                    "u64".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "u_long".to_string(),
                        yukino::ColumnDefinition::new(
                            "u_long".to_string(),
                            yukino::DatabaseType::UnsignedBigInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["u_long".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "id".to_string(),
                yukino::FieldDefinition::new(
                    "id".to_string(),
                    "u32".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "id".to_string(),
                        yukino::ColumnDefinition::new(
                            "id".to_string(),
                            yukino::DatabaseType::UnsignedInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["id".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "short".to_string(),
                yukino::FieldDefinition::new(
                    "short".to_string(),
                    "i16".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "short".to_string(),
                        yukino::ColumnDefinition::new(
                            "short".to_string(),
                            yukino::DatabaseType::SmallInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["short".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "int".to_string(),
                yukino::FieldDefinition::new(
                    "int".to_string(),
                    "i32".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "int".to_string(),
                        yukino::ColumnDefinition::new(
                            "int".to_string(),
                            yukino::DatabaseType::Integer,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["int".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "double".to_string(),
                yukino::FieldDefinition::new(
                    "double".to_string(),
                    "f64".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "double".to_string(),
                        yukino::ColumnDefinition::new(
                            "double".to_string(),
                            yukino::DatabaseType::Double,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["double".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "float".to_string(),
                yukino::FieldDefinition::new(
                    "float".to_string(),
                    "f32".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "float".to_string(),
                        yukino::ColumnDefinition::new(
                            "float".to_string(),
                            yukino::DatabaseType::Float,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["float".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "long".to_string(),
                yukino::FieldDefinition::new(
                    "long".to_string(),
                    "i64".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "long".to_string(),
                        yukino::ColumnDefinition::new(
                            "long".to_string(),
                            yukino::DatabaseType::BigInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["long".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "string".to_string(),
                yukino::FieldDefinition::new(
                    "string".to_string(),
                    "String".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "string".to_string(),
                        yukino::ColumnDefinition::new(
                            "string".to_string(),
                            yukino::DatabaseType::String,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["string".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "u_int".to_string(),
                yukino::FieldDefinition::new(
                    "u_int".to_string(),
                    "u32".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "u_int".to_string(),
                        yukino::ColumnDefinition::new(
                            "u_int".to_string(),
                            yukino::DatabaseType::UnsignedInteger,
                            false,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["u_int".to_string()],
                    None,
                    vec![]
                )
            ),
            (
                "optional".to_string(),
                yukino::FieldDefinition::new(
                    "optional".to_string(),
                    "Option < u32 >".to_string(),
                    false,
                    yukino::DefinitionType::Normal,
                    vec![(
                        "optional".to_string(),
                        yukino::ColumnDefinition::new(
                            "optional".to_string(),
                            yukino::DatabaseType::UnsignedInteger,
                            true,
                            false
                        )
                    )]
                    .into_iter()
                    .collect(),
                    vec!["optional".to_string()],
                    None,
                    vec![]
                )
            )
        ]
        .into_iter()
        .collect(),
        vec![].into_iter().collect(),
        "id".to_string(),
        vec!["id".to_string()]
    );
}
impl YukinoEntity for Basic {
    fn definition() -> &'static EntityDefinition {
        &*BASIC_DEFINITION
    }
    fn entity_id() -> usize {
        0usize
    }
}
impl EntityWithView for Basic {
    type View = BasicView;
    fn all() -> QueryResultFilter<Self> {
        QueryResultFilter::create(&*DEFINITION_MANAGER)
    }
}
impl Value for Basic {
    type L = typenum::U13;
    type ValueExprView = BasicView;
    fn converter() -> ConverterRef<Self>
    where
        Self: Sized,
    {
        BasicConverter::instance()
    }
}
#[derive(Clone)]
pub struct BasicConverter;
impl Converter for BasicConverter {
    type Output = Basic;
    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &Self::INSTANCE
    }
    fn deserializer(&self) -> Deserializer<Self::Output> {
        Box::new(|rest| {
            let (boolean, rest) = Split::<_, typenum::U1>::split(rest);
            let (character, rest) = Split::<_, typenum::U1>::split(rest);
            let (double, rest) = Split::<_, typenum::U1>::split(rest);
            let (float, rest) = Split::<_, typenum::U1>::split(rest);
            let (id, rest) = Split::<_, typenum::U1>::split(rest);
            let (int, rest) = Split::<_, typenum::U1>::split(rest);
            let (long, rest) = Split::<_, typenum::U1>::split(rest);
            let (optional, rest) = Split::<_, typenum::U1>::split(rest);
            let (short, rest) = Split::<_, typenum::U1>::split(rest);
            let (string, rest) = Split::<_, typenum::U1>::split(rest);
            let (u_int, rest) = Split::<_, typenum::U1>::split(rest);
            let (u_long, rest) = Split::<_, typenum::U1>::split(rest);
            let (u_short, _) = Split::<_, typenum::U1>::split(rest);
            Ok(Basic {
                boolean: (*<bool>::converter().deserializer())(boolean)?,
                character: (*<char>::converter().deserializer())(character)?,
                double: (*<f64>::converter().deserializer())(double)?,
                float: (*<f32>::converter().deserializer())(float)?,
                id: (*<u32>::converter().deserializer())(id)?,
                int: (*<i32>::converter().deserializer())(int)?,
                long: (*<i64>::converter().deserializer())(long)?,
                optional: (*<Option<u32>>::converter().deserializer())(optional)?,
                short: (*<i16>::converter().deserializer())(short)?,
                string: (*<String>::converter().deserializer())(string)?,
                u_int: (*<u32>::converter().deserializer())(u_int)?,
                u_long: (*<u64>::converter().deserializer())(u_long)?,
                u_short: (*<u16>::converter().deserializer())(u_short)?,
            })
        })
    }
    fn serialize(
        &self,
        value: &Self::Output,
    ) -> ConvertResult<GenericArray<DatabaseValue, ValueCountOf<Self::Output>>> {
        let boolean = <bool>::converter().serialize(&value.boolean)?;
        let character = <char>::converter().serialize(&value.character)?;
        let double = <f64>::converter().serialize(&value.double)?;
        let float = <f32>::converter().serialize(&value.float)?;
        let id = <u32>::converter().serialize(&value.id)?;
        let int = <i32>::converter().serialize(&value.int)?;
        let long = <i64>::converter().serialize(&value.long)?;
        let optional = <Option<u32>>::converter().serialize(&value.optional)?;
        let short = <i16>::converter().serialize(&value.short)?;
        let string = <String>::converter().serialize(&value.string)?;
        let u_int = <u32>::converter().serialize(&value.u_int)?;
        let u_long = <u64>::converter().serialize(&value.u_long)?;
        let u_short = <u16>::converter().serialize(&value.u_short)?;
        Ok(Concat::concat(
            Concat::concat(
                Concat::concat(
                    Concat::concat(
                        Concat::concat(
                            Concat::concat(
                                Concat::concat(
                                    Concat::concat(
                                        Concat::concat(
                                            Concat::concat(
                                                Concat::concat(
                                                    Concat::concat(
                                                        Concat::concat(
                                                            arr ! [DatabaseValue ;],
                                                            boolean,
                                                        ),
                                                        character,
                                                    ),
                                                    double,
                                                ),
                                                float,
                                            ),
                                            id,
                                        ),
                                        int,
                                    ),
                                    long,
                                ),
                                optional,
                            ),
                            short,
                        ),
                        string,
                    ),
                    u_int,
                ),
                u_long,
            ),
            u_short,
        ))
    }
}
impl ConverterInstance for BasicConverter {
    const INSTANCE: Self = BasicConverter;
}
pub mod basic {
    use yukino::{FieldDefinition, FieldMarker, YukinoEntity};

    use super::Basic;

    #[allow(non_camel_case_types)]
    pub struct boolean();
    impl FieldMarker for boolean {
        type Entity = Basic;
        type FieldType = bool;
        fn field_name() -> &'static str {
            "boolean"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct character();
    impl FieldMarker for character {
        type Entity = Basic;
        type FieldType = char;
        fn field_name() -> &'static str {
            "character"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct double();
    impl FieldMarker for double {
        type Entity = Basic;
        type FieldType = f64;
        fn field_name() -> &'static str {
            "double"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct float();
    impl FieldMarker for float {
        type Entity = Basic;
        type FieldType = f32;
        fn field_name() -> &'static str {
            "float"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct id();
    impl FieldMarker for id {
        type Entity = Basic;
        type FieldType = u32;
        fn field_name() -> &'static str {
            "id"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct int();
    impl FieldMarker for int {
        type Entity = Basic;
        type FieldType = i32;
        fn field_name() -> &'static str {
            "int"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct long();
    impl FieldMarker for long {
        type Entity = Basic;
        type FieldType = i64;
        fn field_name() -> &'static str {
            "long"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct optional();
    impl FieldMarker for optional {
        type Entity = Basic;
        type FieldType = Option<u32>;
        fn field_name() -> &'static str {
            "optional"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct short();
    impl FieldMarker for short {
        type Entity = Basic;
        type FieldType = i16;
        fn field_name() -> &'static str {
            "short"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct string();
    impl FieldMarker for string {
        type Entity = Basic;
        type FieldType = String;
        fn field_name() -> &'static str {
            "string"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_int();
    impl FieldMarker for u_int {
        type Entity = Basic;
        type FieldType = u32;
        fn field_name() -> &'static str {
            "u_int"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_long();
    impl FieldMarker for u_long {
        type Entity = Basic;
        type FieldType = u64;
        fn field_name() -> &'static str {
            "u_long"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
    #[allow(non_camel_case_types)]
    pub struct u_short();
    impl FieldMarker for u_short {
        type Entity = Basic;
        type FieldType = u16;
        fn field_name() -> &'static str {
            "u_short"
        }
        fn definition() -> &'static FieldDefinition {
            Self::Entity::definition()
                .fields
                .get(Self::field_name())
                .unwrap()
        }
    }
}
lazy_static! {
    static ref DEFINITION_MANAGER: DefinitionManager =
        DefinitionManager::create(vec![(0usize, Basic::definition())]);
}
