use yukino::converter::ConverterRef;
use yukino::converter::{ConvertResult, Converter, Deserializer};
use yukino::db::ty::DatabaseValue;
use yukino::err::{RuntimeResult, YukinoError};
use yukino::generic_array::functional::FunctionalSequence;
use yukino::generic_array::sequence::{Concat, Split};
use yukino::generic_array::typenum;
use yukino::generic_array::{arr, GenericArray};
use yukino::interface::Entity;
use yukino::interface::EntityView;
use yukino::query::{Alias, Expr};
use yukino::view::{ExprView, ExprViewBox, SingleExprView, ValueView, ViewBox};
use yukino::view::{Value, View};
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
impl View<Basic, <Basic as Value>::L> for BasicView {
    fn eval(&self, v: &GenericArray<DatabaseValue, <Basic as Value>::L>) -> RuntimeResult<Basic> {
        (*Basic::converter().deserializer())(v).map_err(|e| e.as_runtime_err())
    }
    fn view_clone(&self) -> ViewBox<Basic, <Basic as Value>::L> {
        Box::new(self.clone())
    }
}
impl ValueView<Basic> for BasicView {
    fn collect_expr(&self) -> GenericArray<Expr, <Basic as Value>::L> {
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
}
impl ExprView<Basic> for BasicView {
    fn from_exprs(exprs: GenericArray<Expr, <Basic as Value>::L>) -> Self
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
        BasicView {
            boolean: Box::new(SingleExprView::<bool>::from_exprs(boolean)),
            character: Box::new(SingleExprView::<char>::from_exprs(character)),
            double: Box::new(SingleExprView::<f64>::from_exprs(double)),
            float: Box::new(SingleExprView::<f32>::from_exprs(float)),
            id: Box::new(SingleExprView::<u32>::from_exprs(id)),
            int: Box::new(SingleExprView::<i32>::from_exprs(int)),
            long: Box::new(SingleExprView::<i64>::from_exprs(long)),
            optional: Box::new(SingleExprView::<Option<u32>>::from_exprs(optional)),
            short: Box::new(SingleExprView::<i16>::from_exprs(short)),
            string: Box::new(SingleExprView::<String>::from_exprs(string)),
            u_int: Box::new(SingleExprView::<u32>::from_exprs(u_int)),
            u_long: Box::new(SingleExprView::<u64>::from_exprs(u_long)),
            u_short: Box::new(SingleExprView::<u16>::from_exprs(u_short)),
        }
    }
    fn expr_clone(&self) -> ExprViewBox<Basic>
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
}
impl EntityView for BasicView {
    type Entity = Basic;
    fn pure(alias: &Alias) -> Self
    where
        Self: Sized,
    {
        BasicView {
            boolean: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("boolean" , yukino :: db :: ty :: DatabaseType :: Bool)],
            )),
            character: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("character" , yukino :: db :: ty :: DatabaseType :: Character)],
            )),
            double: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("double" , yukino :: db :: ty :: DatabaseType :: Double)],
            )),
            float: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("float" , yukino :: db :: ty :: DatabaseType :: Float)],
            )),
            id: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("id" , yukino :: db :: ty :: DatabaseType :: UnsignedInteger)],
            )),
            int: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("int" , yukino :: db :: ty :: DatabaseType :: Integer)],
            )),
            long: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("long" , yukino :: db :: ty :: DatabaseType :: BigInteger)],
            )),
            optional: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("optional" , yukino :: db :: ty :: DatabaseType :: UnsignedInteger)],
            )),
            short: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("short" , yukino :: db :: ty :: DatabaseType :: SmallInteger)],
            )),
            string: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("string" , yukino :: db :: ty :: DatabaseType :: String)],
            )),
            u_int: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("u_int" , yukino :: db :: ty :: DatabaseType :: UnsignedInteger)],
            )),
            u_long: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("u_long" , yukino :: db :: ty :: DatabaseType :: UnsignedBigInteger)],
            )),
            u_short: Box::new(SingleExprView::from_exprs(
                arr ! [Expr ; alias . create_ident_expr ("u_short" , yukino :: db :: ty :: DatabaseType :: UnsignedSmallInteger)],
            )),
        }
    }
}
impl Entity for Basic {
    type View = BasicView;
}
impl Value for Basic {
    type L = typenum::U13;
    fn converter() -> ConverterRef<Self>
    where
        Self: Sized,
    {
        BasicConverter::instance()
    }
    fn view(&self) -> ExprViewBox<Self>
    where
        Self: Sized,
    {
        Box::new(BasicView::from_exprs(
            Self::converter().serialize(self).unwrap().map(Expr::Lit),
        ))
    }
}
#[derive(Clone)]
pub struct BasicConverter;
static BASIC_CONVERTER: BasicConverter = BasicConverter;
impl Converter for BasicConverter {
    type Output = Basic;
    fn instance() -> &'static Self
    where
        Self: Sized,
    {
        &BASIC_CONVERTER
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
    ) -> ConvertResult<GenericArray<DatabaseValue, <Self::Output as Value>::L>> {
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
pub mod basic {
    use super::Basic;
    use yukino::interface::def::FieldDefinition;
    use yukino::interface::FieldMarker;
    use yukino::lazy_static::lazy_static;
    #[allow(non_camel_case_types)]
    pub struct boolean();
    lazy_static! {
        static ref BOOLEAN_DEFINITION: FieldDefinition =
            yukino::interface::def::FieldDefinition::new(
                "boolean".to_string(),
                "bool".to_string(),
                false,
                yukino::interface::def::DefinitionType::Normal,
                vec![(
                    "boolean".to_string(),
                    yukino::interface::def::ColumnDefinition::new(
                        "boolean".to_string(),
                        yukino::db::ty::DatabaseType::Bool,
                        false,
                        false
                    )
                )]
                .into_iter()
                .collect(),
                vec!["boolean".to_string()],
                None,
                vec![]
            );
    }
    impl FieldMarker for boolean {
        type Entity = Basic;
        fn field_name() -> &'static str {
            "boolean"
        }
        fn definition() -> &'static FieldDefinition {
            &*BOOLEAN_DEFINITION
        }
    }
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
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
        type Entity = Basic;
        fn field_name() -> &'static str {
            "u_short"
        }
        fn definition() -> &'static FieldDefinition {
            &*U_SHORT_DEFINITION
        }
    }
}
